use ansi::{BgColor, FgColor};
use lines::{FILL_CHAR, set_line_width};

pub struct Iter<I> {
	pub(super) iter: I,
	pub(super) width: usize,
	pub(super) remaining_lines: usize,
	pub(super) bg: Option<BgColor>,
	pub(super) fg: Option<FgColor>,
}
impl<I> ExactSizeIterator for Iter<I>
where
	I: Iterator<Item: Into<String>>,
{
	fn len(&self) -> usize {
		self.remaining_lines
	}
}
impl<I> Iterator for Iter<I>
where
	I: Iterator<Item: Into<String>>,
{
	type Item = String;
	fn next(&mut self) -> Option<Self::Item> {
		(self.remaining_lines > 0).then(|| {
			self.remaining_lines -= 1;
			let line = self
				.iter
				.next()
				.map(|line| set_line_width(&line.into(), self.width))
				.unwrap_or_else(|| FILL_CHAR.repeat(self.width));
			format!(
				"{}{}{}{}",
				self.bg.map(|bg| bg.to_string()).unwrap_or_default(),
				self.fg.map(|fg| fg.to_string()).unwrap_or_default(),
				line,
				ansi::Reset,
			)
		})
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len(), Some(self.len()))
	}
}
