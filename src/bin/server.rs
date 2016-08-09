extern crate tokio;
extern crate japon;

use std::net::SocketAddr;

use tokio::server;
use tokio::tcp::TcpStream;
use tokio::reactor::Reactor;

use japon::Connection;

fn main() {
  let addr: SocketAddr = "127.0.0.1:12345".parse::<SocketAddr>().unwrap();
  let reactor: Reactor= Reactor::default().unwrap();

  server::listen(&reactor.handle(), addr, |stream: TcpStream|
    Ok(Connection::new(stream))
  ).unwrap();

  reactor.run().unwrap();
}
