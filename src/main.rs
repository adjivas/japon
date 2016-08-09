extern crate tokio;
extern crate japon;

use std::net::SocketAddr;

use tokio::server;
use tokio::SimpleService;
use tokio::tcp::TcpStream;
use tokio::reactor::Reactor;

use japon::{Request, Response, Connection};

fn main() {
  let addr = "127.0.0.1:12345".parse::<SocketAddr>().unwrap();
  let service: SimpleService<_, Response> = SimpleService::new(|req: Request| {
    Ok::<Response, Response>(Response { data: req.data })
  });

  let reactor = Reactor::default().unwrap();

  server::listen(&reactor.handle(), addr, |stream: TcpStream|
    Ok(Connection::new(stream))
  );

  reactor.run().unwrap();
//  line::Server::new(addr).serve(service).unwrap()
}
