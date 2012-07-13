import comm::port;
import result::result;
import std::net::tcp::{tcp_err_data, tcp_connect_err_data};
import std::net::ip::ip_addr;

type read_port = port<result<~[u8], tcp_err_data>>;

/**
An abstract client socket connection. This mirrors the bits
of the net::tcp::tcp_socket interface that we care about
while letting us have additional implementations for
mocking
*/
trait connection {
    fn read_start() -> read_port;
    fn read_stop(-read_port: read_port) -> result<(), tcp_err_data>;
}

trait connection_factory<C/*: connection*/> {
    fn connect(ip: ip_addr, port: uint) -> result<C, tcp_connect_err_data>;
}

impl connection for tcp_socket {
    fn read_start() -> result<read_port, tcp_err_data> {
        import std::net::tcp::tcp_socket;
        self.read_start()
    }

    fn read_stop(-read_port: read_port) -> result<(), tcp_err_data> {
        import std::net::tcp::tcp_socket;
        self.read_stop(read_port)
    }
}

type uv_connection_factory = ();

