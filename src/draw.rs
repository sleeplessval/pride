use std::io::{ self, Write };

use termion::{
	terminal_size,

	clear,
	cursor,
	input::TermRead,
	raw::IntoRawMode
};

use crate::color::{ RESET, Colors };
use crate::flag::BLOCK;

pub fn full(colors: Colors) {
	let mut stdout = io::stdout().into_raw_mode().unwrap();
	let stdin = io::stdin();

	let count = colors.len();
	let (width, height) = terminal_size().unwrap();
	let thresh = height as usize / count;

	write!(stdout, "{}{}", cursor::Hide, clear::All).ok();
	stdout.flush().ok();

	let stripe = BLOCK.repeat(width as usize);

	let mut index = 0;
	for n in 0..(height as usize) {
		if n != 0 && n % thresh == 0 {
			index += 1;
			if index >= count { break; }
		}
		write!(
			stdout,
			"{color}{stripe}{RESET}",
			color = colors[index]
		).ok();
	}
	stdout.flush().ok();

	for _ in stdin.keys() { break; }
	write!(stdout, "{}{}", cursor::Show, clear::All).ok();
	stdout.flush().ok();
}

pub fn small(colors: Colors) {
	let mut stdout = io::stdout();

	let count = colors.len();
	let width = count * 3;

	let stripe = BLOCK.repeat(width);

	for color in colors {
		println!("{color}{stripe}");
	}
	print!("{RESET}");
	stdout.flush().ok();
}

