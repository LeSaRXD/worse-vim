pub mod ansi;
pub mod lines;
pub mod window;

use std::{
	io::{BufRead, Write},
	time::Duration,
};

use crate::{lines::Lines, window::Window};

const FILL_CHAR: &str = " ";
const CONTROL_CHAR_WIDTH: usize = 2;
const DEFAULT_SIZE: (usize, usize) = (80, 24);

fn main() {
	let contents = read_lines();

	let mut window = Window::new(contents).bg(10, 0, 30);

	display(&window);
	std::thread::sleep(Duration::MAX);
}

fn display<'a, C>(win: &'a Window<C>)
where
	&'a C: IntoIterator<Item: AsRef<str>>,
{
	let (w, h) = termsize::get()
		.map(|s| (s.cols as usize, s.rows as usize))
		.unwrap_or(DEFAULT_SIZE);
	let lines = win.lines(w, h).collect::<Vec<_>>();

	{
		let mut out = std::io::stdout().lock();
		write!(out, "\x1b[2J{}\x1b[H", lines.join("\n")).ok();
		out.flush().ok();
	}
}

fn read_lines() -> Vec<String> {
	let mut contents = Vec::new();
	let mut line = String::new();
	let mut input = std::io::stdin().lock();
	while let Ok(1..) = input.read_line(&mut line) {
		if line.ends_with('\n') {
			line.pop();
		}
		contents.push(line);
		line = String::new();
	}
	contents
}
