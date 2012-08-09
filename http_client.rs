import to_str::to_str;
import ptr::addr_of;
import comm::{port, chan};
import result::{result, ok, err};
import std::net::ip::{
    get_addr, format_addr, ipv4, ipv6, ip_addr,
    ip_get_addr_err
};
import std::net::tcp::{connect, tcp_socket};
import std::net::url;
import std::net::url::url;
import std::uv_global_loop;
import connection::{
    Connection, ConnectionFactory, UvConnectionFactory,
    MockConnection, MockConnectionFactory
};
import parser::{Parser, ParserCallbacks};
import request::build_request;

const timeout: uint = 2000;

/// HTTP status codes
enum StatusCode {
    StatusOk = 200,
    StatusFound = 302,
    StatusUnknown
}

/// HTTP request error conditions
enum RequestError {
    ErrorDnsResolution,
    ErrorConnect,
    ErrorMisc
}

/// Request 
enum RequestEvent {
    Status(StatusCode),
    Payload(~mut option<~[u8]>),
    Error(RequestError)
}

type DnsResolver = fn@(host: ~str) -> result<~[ip_addr], ip_get_addr_err>;

fn uv_dns_resolver() -> DnsResolver {
    |host| {
        let iotask = uv_global_loop::get();
        get_addr(host, iotask)
    }
}

fn uv_http_request(+url: url) -> HttpRequest<tcp_socket, UvConnectionFactory> {
    HttpRequest(uv_dns_resolver(), UvConnectionFactory, url)
}

#[allow(non_implicitly_copyable_typarams)]
class HttpRequest<C: Connection, CF: ConnectionFactory<C>> {

    let resolve_ip_addr: DnsResolver;
    let connection_factory: CF;
    let url: url;
    let parser: Parser;
    let mut cb: fn@(+RequestEvent);

    new(resolver: DnsResolver, +connection_factory: CF, +url: url) {
        self.resolve_ip_addr = resolver;
        self.connection_factory = connection_factory;
        self.url = url;
        self.parser = Parser();
        self.cb = |_event| { };
    }

    fn begin(cb: fn@(+RequestEvent)) {
        #debug("http_client: looking up url %?", self.url.to_str());
        let ip_addr = match self.get_ip() {
          ok(ip_addr) => { copy ip_addr }
          err(e) => { cb(Error(e)); return }
        };

        #debug("http_client: using IP %? for %?", format_addr(ip_addr), self.url.to_str());

        let socket = {
            #debug("http_client: connecting to %?", ip_addr);
            let socket = self.connection_factory.connect(copy ip_addr, 80);
            if socket.is_ok() {
                result::unwrap(socket)
            } else {
                #debug("http_client: unable to connect to %?: %?", ip_addr, socket);
                cb(Error(ErrorConnect));
                return;
            }
        };

        #debug("http_client: got socket for %?", ip_addr);

        let request_header = build_request(self.url);
        #debug("http_client: writing request header: %?", request_header);
        let request_header_bytes = str::bytes(request_header);
        match socket.write_(request_header_bytes) {
          result::ok(*) => { }
          result::err(e) => {
            // FIXME: Need test
            cb(Error(ErrorMisc));
            return;
          }
        }

        let read_port = {
            let read_port = socket.read_start_();
            if read_port.is_ok() {
                result::unwrap(read_port)
            } else {
                cb(Error(ErrorMisc));
                return;
            }
        };

        // This unsafety is unfortunate but we can't capture self
        // into shared closures
        let unsafe_self = addr_of(self);
        let callbacks: ParserCallbacks = unsafe {{
            on_message_begin: || (*unsafe_self).on_message_begin(),
            on_url: |data| (*unsafe_self).on_url(data),
            on_header_field: |data| (*unsafe_self).on_header_field(data),
            on_header_value: |data| (*unsafe_self).on_header_value(data),
            on_headers_complete: || (*unsafe_self).on_headers_complete(),
            on_body: |data| (*unsafe_self).on_body(data),
            on_message_complete: || (*unsafe_self).on_message_complete()
        }};

        // Set the callback used by the parser event handlers
        self.cb = cb;

        loop {
            let next_data = read_port.recv();

            if next_data.is_ok() {
                let next_data = next_data.get();
                #debug("data: %?", next_data);
                let bytes_parsed = self.parser.execute(next_data, &callbacks);
                if bytes_parsed != next_data.len() {
                    // FIXME: Need tests
                    fail ~"http parse failure";
                }
            } else {
                #debug("http_client: read error: %?", next_data);

                // This method of detecting EOF is lame
                match next_data {
                  result::err({err_name: ~"EOF", _}) => {
                    self.parser.execute(~[], &callbacks);
                    break;
                  }
                  _ => {
                    // FIXME: Need tests and error handling
                    socket.read_stop_(read_port);
                    cb(Error(ErrorMisc));
                    return;
                  }
                }
            }
        }
        socket.read_stop_(read_port);
    }

    fn get_ip() -> result<ip_addr, RequestError> {
        let ip_addrs = self.resolve_ip_addr(self.url.host);
        if ip_addrs.is_ok() {
            let ip_addrs = result::unwrap(ip_addrs);
            // FIXME: This log crashes
            //#debug("http_client: got IP addresses for %?: %?", self.url, ip_addrs);
            if ip_addrs.is_not_empty() {
                // FIXME: Which address should we really pick?
                let best_ip = do ip_addrs.find |ip| {
                    match ip {
                      ipv4(*) => { true }
                      ipv6(*) => { false }
                    }
                };

                if best_ip.is_some() {
                    return ok(option::unwrap(best_ip));
                } else {
                    // FIXME: Need test
                    return err(ErrorMisc);
                }
            } else {
                #debug("http_client: got no IP addresses for %?", self.url);
                // FIXME: Need test
                return err(ErrorMisc);
            }
        } else {
            #debug("http_client: DNS lookup failure: %?", ip_addrs.get_err());
            return err(ErrorDnsResolution);
        }
    }

    fn on_message_begin() -> bool {
        #debug("on_message_begin");
        true
    }

    fn on_url(+_data: ~[u8]) -> bool {
        #debug("on_url");
        true
    }

    fn on_header_field(+data: ~[u8]) -> bool {
        let header_field = str::from_bytes(data);
        #debug("on_header_field: %?", header_field);
        true
    }

    fn on_header_value(+data: ~[u8]) -> bool {
        let header_value = str::from_bytes(data);
        #debug("on_header_value: %?", header_value);
        true
    }

    fn on_headers_complete() -> bool {
        #debug("on_headers_complete");
        true
    }

    fn on_body(+data: ~[u8]) -> bool {
        #debug("on_body");
        let the_payload = Payload(~mut some(data));
        self.cb(the_payload);
        true
    }

    fn on_message_complete() -> bool {
        #debug("on_message_complete");
        true
    }
}

#[allow(non_implicitly_copyable_typarams)]
fn sequence<C: Connection, CF: ConnectionFactory<C>>(request: HttpRequest<C, CF>) -> 
    ~[RequestEvent] {
    
    let events = @mut ~[];
    do request.begin |event| {
        vec::push(*events, event)
    }
    return *events;
}

#[test]
#[allow(non_implicitly_copyable_typarams)]
fn test_resolve_error() {
    let url = url::from_str(~"http://example.com_not_real/").get();
    let request = uv_http_request(url);
    let events = sequence(request);

    assert events == ~[
        Error(ErrorDnsResolution),
    ];
}

#[test]
#[allow(non_implicitly_copyable_typarams)]
fn test_connect_error() {
    // This address is invalid because the first octet
    // of a class A address cannot be 0
    let url = url::from_str(~"http://0.42.42.42/").get();
    let request = uv_http_request(url);
    let events = sequence(request);

    assert events == ~[
        Error(ErrorConnect),
    ];
}

#[test]
#[allow(non_implicitly_copyable_typarams)]
fn test_connect_success() {
    let url = url::from_str(~"http://example.com/").get();
    let request = uv_http_request(url);
    let events = sequence(request);

    for events.each |ev| {
        match ev {
          Error(*) => { fail }
          _ => { }
        }
    }
}

#[test]
#[allow(non_implicitly_copyable_typarams)]
fn test_simple_body() {
    let url = url::from_str(~"http://www.iana.org/").get();
    let request = uv_http_request(url);
    let events = sequence(request);

    let mut found = false;

    for events.each |ev| {
        match ev {
          Payload(value) => {
            if str::from_bytes(value.get()).contains(~"DOCTYPE html") {
                found = true
            }
          }
          _ => { }
        }
    }

    assert found;
}

#[test]
#[ignore(reason = "ICE")]
#[allow(non_implicitly_copyable_typarams)]
fn test_simple_response() {
    let _url = url::from_str(~"http://whatever/").get();
    let _mock_connection: MockConnection = {
        write_fn: |_data| { ok(()) },
        read_start_fn: || {
            let port = port();
            let chan = port.chan();

            let response = ~"HTTP/1.0 200 OK\
                            \
                            Test";
            chan.send(ok(str::bytes(response)));

            ok(port)
        },
        read_stop_fn: |_port| { ok(()) }
    };

    let _mock_connection_factory: MockConnectionFactory = {
        connect_fn: |_ip, _port| {

            // FIXME this doesn't work
            fail;//ok(mock_connection)
        }
    };
}
