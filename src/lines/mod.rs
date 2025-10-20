use unicode_width::UnicodeWidthChar;

use crate::{CONTROL_CHAR_WIDTH, FILL_CHAR};

#[cfg(test)]
pub mod tests;

pub trait Lines {
	type Line;
	fn lines(&self, width: usize, height: usize) -> impl Iterator<Item = Self::Line>;
}

pub fn set_line_width(line: &str, width: usize) -> String {
	let mut split_idx = 0;
	let mut curr_width = 0;

	let (idxs, chars): (Vec<_>, Vec<_>) = line.char_indices().unzip();
	let idxs = idxs.into_iter().skip(1).chain([line.len()]);

	for (idx, ch) in idxs.zip(chars) {
		let char_width = ch.width().unwrap_or(CONTROL_CHAR_WIDTH);
		if curr_width + char_width > width {
			break;
		} else {
			curr_width += char_width;
			split_idx = idx;
		}
	}
	let (prefix, _) = line.split_at(split_idx);
	format!("{prefix}{}", FILL_CHAR.repeat(width - curr_width))
}
