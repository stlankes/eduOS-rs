use core::fmt;
use spin::Mutex;
use core::ptr;
// use aarch64::io::*;

/// A COM serial port.
pub struct ComPort {
	/// COM ports are identified by the base address of their associated
	/// I/O registers.
	port_address: u32
}

impl ComPort {
	/// Create a new COM port with the specified base address.
	pub const fn new(port_address: u32) -> Self {
		Self { port_address }

	}

	pub fn write_byte(&self, byte: u8) {
		let port = self.port_address as *mut u8;

		// LF newline characters need to be extended to CRLF over a real serial port.
		if byte == b'\n' {
			unsafe { ptr::write_volatile(port, b'\r'); }
		}

		unsafe { ptr::write_volatile(port, byte); }
	}
}

impl fmt::Write for ComPort {
	/// Output a string to our COM port.  This allows using nice,
	/// high-level tools like Rust's `write!` macro.
	fn write_str(&mut self, s: &str) -> fmt::Result {
		// Output each byte of our string.
		for &b in s.as_bytes() {
			// Write our byte.
			self.write_byte(b);
		}
		Ok(())
	}
}

/// Our primary serial port.
pub static COM1: Mutex<ComPort> = Mutex::new(ComPort::new(0x3F8));
