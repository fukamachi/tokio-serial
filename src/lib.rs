//! Bindings for serial port I/O and futures
//!
//! This crate provides bindings between `mio_serial`, a mio crate for
//! serial port I/O, and `futures`.  The API is very similar to the
//! bindings in `mio_serial`
//!
//! Currently only unix-based platforms are supported.  This is not
//! a technical limitation within rust and will hopefully change in
//! future releases.

// For now we provide only implementations for unix/termios
#![deny(missing_docs)]

#[macro_use]
extern crate futures;
#[macro_use]
extern crate tokio_core;

extern crate mio;
extern crate mio_serial;

// Re-export serialport types and traits from mio_serial
pub use mio_serial::{BaudRate, DataBits, StopBits, FlowControl, Parity, SerialPort,
                     SerialPortSettings, SerialResult};

#[cfg(unix)]
pub use unix::Serial;
#[cfg(windows)]
pub use windows::Serial;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;
