//! A wrapper around our serial console.

use core::fmt;
use arch::serial;

pub struct Console;

impl fmt::Write for Console {
	/// Output a string to each of our console outputs.
	fn write_str(&mut self, s: &str) -> fmt::Result {
		unsafe { serial::COM1.write_str(s) }
	}
}
