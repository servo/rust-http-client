use std::net::url::Url;

pub fn build_request(url: Url) -> ~str {

    let host = copy url.host;
    let mut path = if url.path.len() > 0 { copy url.path } else { ~"/" };

    if url.query.len() > 0 {
        let kvps = do url.query.map |pair| {
            match *pair {
                (ref key, ref value) => fmt!("%s=%s", *key, *value)
            }
        };
        path += ~"?" + str::connect(kvps, "&");
    }

    let request_header = fmt!("GET %s HTTP/1.0\u000D\u000AHost: %s\u000D\u000A\u000D\u000A",
                              path, host);

    return request_header;
}

#[test]
#[allow(non_implicitly_copyable_typarams)]
fn should_request_slash_when_path_is_empty() {
    let url = url::from_str(~"http://host").get();
    assert!(url.path.is_empty());
    let headers = build_request(url);
    assert!(headers.contains(~"GET / "));
}
