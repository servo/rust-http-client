import comm::port;
import result::result;
import std::net::tcp::{tcp_err_data, tcp_connect_err_data};
import std::net::ip::ip_addr;

type ReadPort = port<result<~[u8], tcp_err_data>>;

/**
An abstract client socket connection. This mirrors the bits
of the net::tcp::tcp_socket interface that we care about
while letting us have additional implementations for
mocking
*/
trait Connection {
    fn read_start() -> ReadPort;
    fn read_stop(-read_port: ReadPort) -> result<(), tcp_err_data>;
}

trait ConnectionFactory<C/*: connection*/> {
    fn connect(ip: ip_addr, port: uint) -> result<C, tcp_connect_err_data>;
}

impl Connection for tcp_socket {
    fn read_start() -> result<ReadPort, tcp_err_data> {
        import std::net::tcp::tcp_socket;
        self.read_start()
    }

    fn read_stop(-read_port: ReadPort) -> result<(), tcp_err_data> {
        import std::net::tcp::tcp_socket;
        self.read_stop(read_port)
    }
}

type UvConnectionFactory = ();

