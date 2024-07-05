//!	render handling code

use std::io::{
	self,
	Write
};

use termion::{
	terminal_size,

	clear,
	color::{ Bg, Fg, Rgb },
	cursor,
	input::TermRead,
	raw::{ RawTerminal, IntoRawMode }
};

use crate::{
	color::{ RESET, RESET_BG },
	error,
	flag::Flag,
	state::{ Size, State }
};

pub static BLOCK: &str = "█";
pub static UHALF: &str = "▀";

///	prints a provided vec of lines to stdout
pub fn draw_lines(lines: Vec<String>, state: &State) {
	let mut stdout = io::stdout().into_raw_mode().unwrap();

	let count = lines.len() as u16;
	for _ in 0..count { write!(stdout, "\n").ok(); }
	write!(stdout, "{}", cursor::Up(count)).ok();

	let hold = state.size == Size::Full;
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

///	generates lines for foreground colors provided as a vec of strings for the draw_lines method
pub fn fg_stripes(colors: Vec<Fg<Rgb>>, width: u16, height: u16) -> Vec<String> {
	let width = width as usize;
	let height = height as usize;
	let count = colors.len();

	let thresh = height / count;

	let stripe = BLOCK.repeat(width);
	let mut output = Vec::new();

	//	create our color index
	let mut index = 0;
	for n in 0..height {
		if n != 0 && n % thresh == 0 {
			index += 1;
			//	and break if out of bounds
			if index >= count { break; }
		}
		let color = colors[index];
		output.push(format!("{color}{stripe}"));
	}

	output
}
///	generates lines for background colors provided as a vec of strings for the draw_lines method
pub fn bg_stripes(colors: Vec<Bg<Rgb>>, width: u16, height: u16) -> Vec<String> {
	let width = width as usize;
	let height = height as usize;
	let count = colors.len();

	let thresh = height / count;

	let stripe = " ".repeat(width);
	let mut output = Vec::new();

	let mut index = 0;
	for n in 0..height {
		if n != 0 && n % thresh == 0 {
			index += 1;
			if index >= count { break; }
		}
		let color = colors[index];
		output.push(format!("{color}{stripe}"));
	}

	output
}

impl Flag {
	///	renders a flag to stdout
	pub fn draw(self, state: &State) {
		let lines = match self {
			Flag::Stripes(colors)
				=> {
					let count = colors.len() as u16;
					let (width, height) = state.size.get(count * 3, count);
					if height < count { error::too_small(width, height); }
					fg_stripes(colors, width, height)
				},
			Flag::Lines(lines)
				=>	lines
		};
		draw_lines(lines, &state);
	}
}

