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
    fn write(data: ~[u8]) -> result<(), tcp_err_data>;
    fn read_start() -> result<ReadPort, tcp_err_data>;
    fn read_stop(-read_port: ReadPort) -> result<(), tcp_err_data>;
}

trait ConnectionFactory<C: Connection> {
    fn connect(ip: ip_addr, port: uint) -> result<C, tcp_connect_err_data>;
}

impl of Connection for tcp_socket {
    fn write(data: ~[u8]) -> result<(), tcp_err_data> {
        import std::net::tcp::tcp_socket;
        self.write(data)
    }

    fn read_start() -> result<ReadPort, tcp_err_data> {
        import std::net::tcp::tcp_socket;
        self.read_start()
    }

    fn read_stop(-read_port: ReadPort) -> result<(), tcp_err_data> {
        import std::net::tcp::tcp_socket;
        self.read_stop(read_port)
    }
}

enum UvConnectionFactory {
    UvConnectionFactory
}

impl of ConnectionFactory<tcp_socket> for UvConnectionFactory {
    fn connect(ip: ip_addr, port: uint) -> result<tcp_socket, tcp_connect_err_data> {
        import std::uv_global_loop;
        import std::net::tcp::connect;
        let iotask = uv_global_loop::get();
        ret connect(copy ip, port, iotask);
    }
}

type MockConnection = {
    write_fn: fn@(~[u8]) -> result<(), tcp_err_data>,
    read_start_fn: fn@() -> result<ReadPort, tcp_err_data>,
    read_stop_fn: fn@(-ReadPort) -> result<(), tcp_err_data>
};

impl of Connection for MockConnection {
    fn write(data: ~[u8]) -> result<(), tcp_err_data> {
        self.write_fn(data)
    }

    fn read_start() -> result<ReadPort, tcp_err_data> {
        self.read_start_fn()
    }

    fn read_stop(-read_port: ReadPort) -> result<(), tcp_err_data> {
        self.read_stop_fn(read_port)
    }
}

type MockConnectionFactory = {
    connect_fn: fn@(ip_addr, uint) -> result<MockConnection, tcp_connect_err_data>
};

impl of ConnectionFactory<MockConnection> for MockConnectionFactory {
    fn connect(ip: ip_addr, port: uint) -> result<MockConnection, tcp_connect_err_data> {
        self.connect_fn(ip, port)
    }
}
