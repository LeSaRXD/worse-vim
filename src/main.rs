use std::{
	io::{BufRead, Write},
	time::Duration,
};

use lines::Lines;
use window::{Split, Window};

const DEFAULT_SIZE: (usize, usize) = (80, 24);

fn main() {
	let contents1 = read_lines();
	let contents2 = read_lines();
	let contents3 = read_lines();

	let windows = vec![
		Window::new(&contents1).bg(10, 0, 30),
		Window::new(&contents2).bg(30, 10, 0),
		Window::new(&contents3).bg(0, 30, 10),
	];
	let split = Split::Horizontal(windows);

	display(&split);
	std::thread::sleep(Duration::MAX);
}

fn display<L>(into_lines: &L)
where
	L: Lines<Line = String>,
{
	let (w, h) = termsize::get()
		.map(|s| (s.cols as usize, s.rows as usize))
		.unwrap_or(DEFAULT_SIZE);
	let lines = into_lines.lines(w, h).collect::<Vec<_>>();

	{
		let mut out = std::io::stdout().lock();
		write!(out, "{}{}{}", ansi::Clear, lines.join("\n"), ansi::Home).ok();
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
