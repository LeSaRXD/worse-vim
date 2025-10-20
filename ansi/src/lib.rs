mod color;
use std::fmt::Display;

pub use color::*;

pub struct Reset;
impl Display for Reset {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("\x1b[0m")
	}
}

pub struct Clear;
impl Display for Clear {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("\x1b[2J")
	}
}

pub struct Home;
impl Display for Home {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("\x1b[H")
	}
}
