import comm::Port;
import std::net::tcp::{TcpErrData, TcpConnectErrData};
import std::net::ip::IpAddr;

type ReadPort = Port<Result<~[u8], TcpErrData>>;

/**
An abstract client socket connection. This mirrors the bits
of the net::tcp::TcpSocket interface that we care about
while letting us have additional implementations for
mocking
*/
trait Connection {
    fn write_(data: ~[u8]) -> Result<(), TcpErrData>;
    fn read_start_() -> Result<ReadPort, TcpErrData>;
    fn read_stop_(-read_port: ReadPort) -> Result<(), TcpErrData>;
}

trait ConnectionFactory<C: Connection> {
    fn connect(ip: IpAddr, port: uint) -> Result<C, TcpConnectErrData>;
}

impl TcpSocket : Connection {
    fn write_(data: ~[u8]) -> Result<(), TcpErrData> {
        import std::net::tcp::TcpSocket;
        self.write(data)
    }

    fn read_start_() -> Result<ReadPort, TcpErrData> {
        import std::net::tcp::TcpSocket;
        self.read_start()
    }

    fn read_stop_(-read_port: ReadPort) -> Result<(), TcpErrData> {
        import std::net::tcp::TcpSocket;
        self.read_stop(read_port)
    }
}

enum UvConnectionFactory {
    UvConnectionFactory
}

impl UvConnectionFactory : ConnectionFactory<TcpSocket> {
    fn connect(ip: IpAddr, port: uint) -> Result<TcpSocket, TcpConnectErrData> {
        import std::uv_global_loop;
        import std::net::tcp::connect;
        let iotask = uv_global_loop::get();
        return connect(copy ip, port, iotask);
    }
}

type MockConnection = {
    write_fn: fn@(~[u8]) -> Result<(), TcpErrData>,
    read_start_fn: fn@() -> Result<ReadPort, TcpErrData>,
    read_stop_fn: fn@(-ReadPort) -> Result<(), TcpErrData>
};

impl MockConnection : Connection {
    fn write_(data: ~[u8]) -> Result<(), TcpErrData> {
        self.write_fn(data)
    }

    fn read_start_() -> Result<ReadPort, TcpErrData> {
        self.read_start_fn()
    }

    fn read_stop_(-read_port: ReadPort) -> Result<(), TcpErrData> {
        self.read_stop_fn(read_port)
    }
}

type MockConnectionFactory = {
    connect_fn: fn@(IpAddr, uint) -> Result<MockConnection, TcpConnectErrData>
};

impl MockConnectionFactory : ConnectionFactory<MockConnection> {
    fn connect(ip: IpAddr, port: uint) -> Result<MockConnection, TcpConnectErrData> {
        self.connect_fn(ip, port)
    }
}
