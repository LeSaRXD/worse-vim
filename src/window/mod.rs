pub mod iter;
use iter::Iter;

use crate::{ansi::Color, lines::Lines};

pub struct Window<C> {
	pub content: C,
	pub bg: Option<Color>,
	pub fg: Option<Color>,
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
		self.fg = Some(Color(r, g, b));
		self
	}
	pub fn bg(mut self, r: u8, g: u8, b: u8) -> Self {
		self.bg = Some(Color(r, g, b));
		self
	}

	pub fn inner_mut(&mut self) -> &mut C {
		&mut self.content
	}
}
impl<'a, C> Lines for &'a Window<C>
where
	&'a C: IntoIterator<Item: AsRef<str>>,
{
	type Line = String;
	fn lines(self, width: usize, height: usize) -> impl Iterator<Item = Self::Line> {
		Iter {
			iter: self.content.into_iter(),
			width,
			remaining_lines: height,
			fg: self.fg,
			bg: self.bg,
		}
	}
}
