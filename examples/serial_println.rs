extern crate futures;
extern crate tokio_serial;
extern crate tokio_core;

use std::{io, env, str};
use tokio_core::io::{Io, Codec, EasyBuf};
use tokio_core::reactor::Core;
use futures::{future, Future, Stream, Sink};

struct LineCodec;

impl Codec for LineCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        let newline = buf.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = buf.drain_to(n + 1);
            return match str::from_utf8(&line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }

    // Don't actually encode anything.
    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        Ok(())
    }
}

struct Printer {
    serial: tokio_serial::Serial,
    buf: Vec<u8>,
}


fn main() {

    let mut args = env::args();
    let tty_path = args.nth(1).unwrap_or("/dev/ttyUSB0".into());

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let settings = tokio_serial::SerialPortSettings::default();
    let mut port = tokio_serial::Serial::from_path(tty_path, &settings, &handle).unwrap();
    port.set_exclusive(false).expect("Unable to set serial port exlusive");

    let (_, reader) = port.framed(LineCodec).split();

    let printer = reader.for_each(|s| {
        println!("{:?}", s);
        Ok(())
    });


    core.run(printer).unwrap();

}
