use core::fmt;
use core::ptr::write_volatile;

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
}

impl fmt::Write for ComPort {
	/// Output a string to our COM port.  This allows using nice,
	/// high-level tools like Rust's `write!` macro.
	fn write_str(&mut self, s: &str) -> fmt::Result {
		// Output each byte of our string.
		for &b in s.as_bytes() {
			// Write our byte.
			unsafe { write_volatile(self.port_address as *mut u8, b); }
		}
		Ok(())
	}
}

/// Our primary serial port.
pub static mut COM1: ComPort = ComPort::new(0x800 /*0x09000000*/);
