use crate::{FILL_CHAR, lines::set_line_width};

pub struct Iter<I> {
	pub(super) iter: I,
	pub(super) width: usize,
	pub(super) remaining_lines: usize,
	pub(super) bg: Option<super::Color>,
	pub(super) fg: Option<super::Color>,
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
				"{}{}{}\x1b[0m",
				self.bg
					.map(|bg| format!("\x1b[48;2;{};{};{}m", bg.0, bg.1, bg.2))
					.unwrap_or_default(),
				self.fg
					.map(|fg| format!("\x1b[38;2;{};{};{}m", fg.0, fg.1, fg.2))
					.unwrap_or_default(),
				line
			)
		})
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len(), Some(self.len()))
	}
}
