mod split;
pub use split::*;
pub mod iter;

use ansi::{BgColor, Color, FgColor};
use iter::Iter;
use lines::Lines;

pub struct Window<C> {
	pub content: C,
	pub bg: Option<BgColor>,
	pub fg: Option<FgColor>,
}

impl<C> Window<C> {
	pub fn new(content: C) -> Self {
		Self {
			content,
			bg: None,
			fg: None,
		}
	}
	pub fn fg(mut self, r: u8, g: u8, b: u8) -> Self {
		self.fg = Some(Color(r, g, b).into());
		self
	}
	pub fn bg(mut self, r: u8, g: u8, b: u8) -> Self {
		self.bg = Some(Color(r, g, b).into());
		self
	}

	pub fn inner_mut(&mut self) -> &mut C {
		&mut self.content
	}
}
impl<'a, C> Lines for Window<&'a C>
where
	&'a C: IntoIterator<Item: Into<String>>,
{
	type Line = String;
	fn lines(&self, width: usize, height: usize) -> impl Iterator<Item = Self::Line> {
		Iter {
			iter: self.content.into_iter(),
			width,
			remaining_lines: height,
			fg: self.fg,
			bg: self.bg,
		}
	}
}
