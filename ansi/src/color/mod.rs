use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);
impl From<(u8, u8, u8)> for Color {
	fn from((r, g, b): (u8, u8, u8)) -> Self {
		Self(r, g, b)
	}
}
impl Color {
	pub fn new(r: u8, g: u8, b: u8) -> Self {
		Self(r, g, b)
	}
	pub fn fg(self) -> FgColor {
		self.into()
	}
	pub fn bg(self) -> BgColor {
		self.into()
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FgColor(pub Color);
impl From<Color> for FgColor {
	fn from(value: Color) -> Self {
		Self(value)
	}
}
impl Display for FgColor {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\x1b[38;2;{};{};{}m", self.0.0, self.0.1, self.0.2)
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct BgColor(pub Color);
impl From<Color> for BgColor {
	fn from(value: Color) -> Self {
		Self(value)
	}
}
impl Display for BgColor {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\x1b[48;2;{};{};{}m", self.0.0, self.0.1, self.0.2)
	}
}
