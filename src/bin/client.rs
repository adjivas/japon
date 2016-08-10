extern crate tokio;
extern crate japon;

use std::net::SocketAddr;

use tokio::client;
use tokio::tcp::TcpStream;
use tokio::reactor::Reactor;

use japon::Service;

fn main() {
  let addr: SocketAddr = "127.0.0.1:12345".parse::<SocketAddr>().unwrap();
  let reactor: Reactor= Reactor::default().unwrap();

  client::connect(&reactor.handle(), addr, |stream: TcpStream|
    Ok(Service::new(stream))
  );

  reactor.run().unwrap();
}
