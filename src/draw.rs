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

///	draw a fullscreen stripe flag and hold for keypress
pub fn full(colors: Colors) {
	//	prepare stdin and stdout
	let mut stdout = io::stdout().into_raw_mode().unwrap();
	let stdin = io::stdin();

	//	get constraints
	let count = colors.len();
	let (width, height) = terminal_size().unwrap();
	let thresh = height as usize / count;

	//	clear the terminal
	write!(stdout, "{}{}", cursor::Hide, clear::All).ok();
	stdout.flush().ok();

	//	build terminal width stripe string
	let stripe = BLOCK.repeat(width as usize);

	//	create our color index
	let mut index = 0;
	//	for every terminal row...
	for n in 0..(height as usize) {
		//	... increment our index at color change threshold
		if n != 0 && n % thresh == 0 {
			index += 1;
			//	and break if out of bounds
			if index >= count { break; }
		}
		//	... draw the stripe with color at index
		write!(
			stdout,
			"{color}{stripe}{RESET}",
			color = colors[index]
		).ok();
	}
	//	flush stdout
	stdout.flush().ok();

	//	wait for keypress
	for _ in stdin.keys() { break; }
	write!(stdout, "{}{}", cursor::Show, clear::All).ok();
	stdout.flush().ok();
}

///	draws a small stripe flag
pub fn small(colors: Colors) {
	//	prepare stdout
	let mut stdout = io::stdout();

	//	get constraints
	let count = colors.len();
	let width = count * 3;

	// build small stripe string
	let stripe = BLOCK.repeat(width);

	//	print a stripe for all colors
	for color in colors {
		println!("{color}{stripe}");
	}
	//	reset our foreground color to play nice and flush stdout
	print!("{RESET}");
	stdout.flush().ok();
}

