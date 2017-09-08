use redirect::Proxy;
use futures::Future;
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;
use std::io;
use std::net::SocketAddr;
use socks::FutureExt;

pub struct Direct {
    addr: SocketAddr,
}

impl Direct {
    pub fn new(addr: SocketAddr) -> Direct {
        Direct { addr }
    }
}

impl Proxy for Direct {
    fn connect(
        &self,
        handle: Handle,
    ) -> Box<Future<Item = TcpStream, Error = io::Error> + 'static> {
        TcpStream::connect(&self.addr, &handle).into_box()
    }
}