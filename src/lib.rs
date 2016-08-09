extern crate tokio;

use std::io::Result;

use tokio::io::TryRead;
use tokio::tcp::TcpStream;
use tokio::reactor::{Task, Tick};

pub struct Response {
    pub data: [u8; 80],
}

pub struct Request {
    pub data: [u8; 80],
}

pub struct Connection {
    stream: TcpStream,
    buf: Box<[u8]>,
}

impl Connection {
    pub fn new (
        stream: TcpStream,
    ) -> Self {
        Connection {
            stream: stream,
            buf: vec![0; 1024].into_boxed_slice(),
        }
    }
}

impl Task for Connection {
    fn tick(&mut self) -> Result<Tick> {
        while let Some(n) = try!(self.stream.try_read(&mut self.buf)) {
            if n == 0 {
                return Ok(Tick::Final);
            }
            else {
                println!("read {} bytes", n);
            }
        }
        Ok(Tick::WouldBlock)
    }
}
