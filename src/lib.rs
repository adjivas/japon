extern crate tokio;

use std::io;

use tokio::io::TryRead;
use tokio::io::{Readiness, Transport};
use tokio::tcp::TcpStream;
use tokio::reactor::{Task, Tick};
use tokio::proto::pipeline;

pub struct Service {
    stream: TcpStream,
    buf: Box<[u8]>,
}

impl Service {
    pub fn new (
        stream: TcpStream,
    ) -> Self {
        Service {
            stream: stream,
            buf: vec![0; 1024].into_boxed_slice(),
        }
    }
}

impl Task for Service {
    fn tick(&mut self) -> io::Result<Tick> {
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

pub type Frame = pipeline::Frame<String, io::Error>;

impl Transport for Service {
  type In = Frame;
  type Out = Frame;

  fn read (
    &mut self,
  ) -> io::Result<Option<Frame>> {
    println!("Hello read");
    Ok(Some(pipeline::Frame::Done))
  }

  fn write(&mut self, _: Frame) -> io::Result<Option<()>> {
    println!("Hello write");
    Ok(Some(()))   
  }

  fn flush(&mut self) -> io::Result<Option<()>> {
    println!("Hello flush");
    Ok(Some(()))
  }
}

impl Readiness for Service {
  fn is_readable(&self) -> bool {
    true
  }

  fn is_writable(&self) -> bool {
    true
  }
}
