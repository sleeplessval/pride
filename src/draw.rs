use std::io::{ self, Write };

use termion::{
	terminal_size,

	clear,
	cursor,
	input::TermRead,
	raw::IntoRawMode
};

use crate::color::{ RESET, RESET_BG, Colors };

pub static BLOCK: &str = "█";
pub static UHALF: &str = "▀";

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

pub fn lines(lines: Vec<String>, hold: bool) {
	let mut stdout = io::stdout().into_raw_mode().unwrap();

	let count = lines.len() as u16;
	for _ in 0..count { write!(stdout, "\n").ok(); }
	write!(stdout, "{}", cursor::Up(count)).ok();

	if hold { write!(stdout, "{}{}", cursor::Hide, clear::All).ok(); }

	let down = cursor::Down(1);
	for line in lines {
		let left = cursor::Left(line.len() as u16);
		write!(stdout, "{line}{left}{down}").ok();
	}

	write!(stdout, "{RESET}{RESET_BG}").ok();
	stdout.flush().ok();
	if hold {
		let stdin = io::stdin();
		for _ in stdin.keys() { break; }
		write!(stdout, "{}", clear::All).ok();
	}
	write!(stdout, "{}", cursor::Show).ok();
	stdout.flush().ok();
}

