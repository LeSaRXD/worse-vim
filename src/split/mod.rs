pub mod iter;
#[cfg(test)]
mod tests;

use std::iter::repeat_n;

use crate::{lines::Lines, split::iter::SplitIter};

pub enum Split<L> {
	Single(L),
	Horizontal(Vec<L>),
	Vertical(Vec<L>),
}

impl<L> Lines for Split<L>
where
	L: Lines<Line: Into<String>>,
{
	type Line = String;
	fn lines(&self, width: usize, height: usize) -> impl Iterator<Item = Self::Line> {
		match self {
			Self::Single(lines) => SplitIter::single(lines.lines(width, height)),
			Self::Horizontal(lines_list) => {
				let line_widths = divide_into_intervals(width, lines_list.len());
				let lines = lines_list
					.iter()
					.zip(line_widths)
					.map(|(into_lines, width)| into_lines.lines(width, height))
					.collect();
				SplitIter::horizontal(lines)
			}
			Self::Vertical(lines_list) => {
				let gine_heights = divide_into_intervals(height, lines_list.len());
				let lines = lines_list
					.iter()
					.zip(gine_heights)
					.map(|(into_lines, height)| into_lines.lines(width, height))
					.collect();
				SplitIter::vertical(lines)
			}
		}
	}
}

fn divide_into_intervals(big_interval: usize, count: usize) -> impl Iterator<Item = usize> {
	let smaller_interval = big_interval / count;
	let num_bigger = big_interval % count;
	repeat_n(smaller_interval + 1, num_bigger).chain(repeat_n(smaller_interval, count - num_bigger))
}
