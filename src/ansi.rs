#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);
impl From<(u8, u8, u8)> for Color {
	fn from((r, g, b): (u8, u8, u8)) -> Self {
		Self(r, g, b)
	}
}
impl Color {
	pub fn fg(&self) -> String {
		format!("\x1b[38;2;{};{};{}m", self.0, self.1, self.2)
	}
	pub fn bg(&self) -> String {
		format!("\x1b[48;2;{};{};{}m", self.0, self.1, self.2)
	}
}
