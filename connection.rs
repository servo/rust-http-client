// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::comm::Port;
use extra::net::tcp::{TcpErrData, TcpConnectErrData, TcpSocket};
use extra::net::ip::IpAddr;

pub type ReadPort = Port<Result<~[u8], TcpErrData>>;

/**
An abstract client socket connection. This mirrors the bits
of the net::tcp::TcpSocket interface that we care about
while letting us have additional implementations for
mocking
*/
pub trait Connection {
    fn write_(&self, data: ~[u8]) -> Result<(), TcpErrData>;
    fn read_start_(&self) -> Result<@ReadPort, TcpErrData>;
    fn read_stop_(&self, read_port: @ReadPort) -> Result<(), TcpErrData>;
}

pub trait ConnectionFactory<C: Connection> {
    fn connect(&self, ip: IpAddr, port: uint) -> Result<C, TcpConnectErrData>;
}

impl Connection for TcpSocket {
    fn write_(&self, data: ~[u8]) -> Result<(), TcpErrData> {
        self.write(data)
    }

    fn read_start_(&self) -> Result<@ReadPort, TcpErrData> {
        self.read_start()
    }

    fn read_stop_(&self, _read_port: @ReadPort) -> Result<(), TcpErrData> {
        self.read_stop()
    }
}

pub enum UvConnectionFactory {
    UvConnectionFactory
}

impl ConnectionFactory<TcpSocket> for UvConnectionFactory {
    fn connect(&self, ip: IpAddr, port: uint) -> Result<TcpSocket, TcpConnectErrData> {
        use extra::uv_global_loop;
        use extra::net::tcp::connect;
        let iotask = uv_global_loop::get();
        return connect(copy ip, port, &iotask);
    }
}

pub struct MockConnection {
    write_fn: @fn(~[u8]) -> Result<(), TcpErrData>,
    read_start_fn: @fn() -> Result<@ReadPort, TcpErrData>,
    read_stop_fn: @fn(port: @ReadPort) -> Result<(), TcpErrData>
}

impl Connection for MockConnection {
    fn write_(&self, data: ~[u8]) -> Result<(), TcpErrData> {
        (self.write_fn)(data)
    }

    fn read_start_(&self) -> Result<@ReadPort, TcpErrData> {
        (self.read_start_fn)()
    }

    fn read_stop_(&self, read_port: @ReadPort) -> Result<(), TcpErrData> {
        (self.read_stop_fn)(read_port)
    }
}

pub struct MockConnectionFactory {
    connect_fn: @fn(IpAddr, uint) -> Result<MockConnection, TcpConnectErrData>
}

impl ConnectionFactory<MockConnection> for MockConnectionFactory {
    fn connect(&self, ip: IpAddr, port: uint) -> Result<MockConnection, TcpConnectErrData> {
        (self.connect_fn)(ip, port)
    }
}
