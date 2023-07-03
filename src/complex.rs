use std::process::exit;

use termion::{
	terminal_size,

	color::{ Bg, Rgb }
};

use crate::color::*;
use crate::draw;
use crate::flag;

///	vertically stacking eighths
pub static V_EIGHTH: [char; 7] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇'];
///	horizontally stacking eighths
pub static H_EIGHTH: [char; 7] = ['▏', '▎', '▍', '▌', '▋', '▊', '▉'];

///	shading by intensity
pub static SHADING: [char; 3] = ['░', '▒', '▓'];

///	2/1 slope triangle cut in
pub static TRIANGLE_21: [char; 3] = ['', '🭬', ''];

///	2/3 slope slant
pub static SLANT_23: [char; 2] = ['🭒', '🭏'];

pub fn progress(small: bool) -> Colors {
	let red		= bg(0xE50000);
	let orange	= bg(0xFF8D00);
	let yellow	= bg(0xFFEE00);
	let green	= bg(0x028121);
	let blue	= bg(0x004CFF);
	let purple	= bg(0x770088);

	//	we need these colors in both fg & bg; just hold the integers for now
	let black:	u32 = 0;
	let brown:	u32 = 0x784F17;
	let pink:	u32 = 0xEAACB8;
	let white:	u32 = 0xFFFFFF;

	let (width, height) = if small { (6, 18) } else { terminal_size().unwrap() };

	let stripes = vec![red, orange, yellow, green, blue, purple];
	let mut lines = draw::bg_stripes(stripes, width, height);

	draw::lines(lines, !small);
	exit(0);
}

//	everything below this point is in alphabetical order

pub fn aroace() {
	let aro = flag::aromantic();
	let ace = flag::asexual();

}

fn demi_orientation_render(middle: Bg<Rgb>, bottom: Bg<Rgb>, width: u16, height: u16) -> Vec<String> {
	let white	= bg(0xFFFFFF);

	let stripes = vec![white, white, middle, bottom, bottom];

	//	initial stripe output buffer
	let mut lines: Vec<String> = draw::bg_stripes(stripes, width, height);

	//	assemble triangle cut-in
	let linecount = lines.len();
	let depth = linecount / 2;
	let corner = linecount % 2 == 1;
	for n in 0..depth {
		let line = lines[n].clone();

		let replacement = format!("{BLACK}{}{}", draw::BLOCK.repeat(n), TRIANGLE_21[0]);
		lines[n] = line.replacen(&" ".repeat(n + 1), &replacement, 1);
	}
	if corner {
		let line = lines[depth].clone();

		let replacement = format!("{BLACK}{}{}", draw::BLOCK.repeat(depth), TRIANGLE_21[1]);
		lines[depth] = line.replacen(&" ".repeat(depth + 1), &replacement, 1);
	}
	let start = depth + (linecount % 2);
	for n in 0..depth {
		let line = lines[start + n].clone();

		let size = depth - n - 1;
		let replacement = format!("{BLACK}{}{}", draw::BLOCK.repeat(size), TRIANGLE_21[2]);
		lines[start + n] = line.replacen(&" ".repeat(size + 1), &replacement, 1);
	}

	lines
}

pub fn demiromantic(small: bool) -> Colors {
	let green	= bg(0x3DA542);
	let gray	= bg(0xD2D2D2);

	let (width, height) = if small { (15, 5) } else { terminal_size().unwrap() };
	let lines = demi_orientation_render(green, gray, width, height);

	draw::lines(lines, !small);
	exit(0);
}

pub fn demisexual(small: bool) -> Colors {
	let purple	= bg(0x832FA8);
	let grey	= bg(0x7B868C);

	let (width, height) = if small { (15, 5) } else { terminal_size().unwrap() };
	let lines = demi_orientation_render(purple, grey, width, height);

	draw::lines(lines, !small);
	exit(0);
}

pub fn disability() {
	let gray	= bg(0x575757);

	let green:	u32 = 0x3AAD7D;
	let blue:	u32 = 0x79BFE0;
	let white:	u32 = 0xE8E8E8;
	let yellow:	u32 = 0xEDDB76;
	let red:	u32 = 0xCD7281;

	let stripes = [red, yellow, white, blue, green];

	// 2/3 slant stripes with gray background
}

pub fn intersex() -> Colors {
	let yellow	= bg(0xFFDA00);
	let purple	= rgb(0x7A00AC);

	let block = " ";
	let stripe = block.repeat(9);
	let part = block.repeat(4);

	let lines = vec![
		format!("{yellow}{stripe}"),
		format!("{part}{purple}O{part}"),
		format!("{stripe}")
	];

	draw::lines(lines, false);
	exit(0);
}

pub fn polyamorous() {
	let blue	= rgb(0x019FE3);
	let magenta	= rgb(0xE50051);
	let purple	= rgb(0x340C46);
	let yellow	= rgb(0x00FCBF);

	//	blue / magenta / purple vert
	//	WHITE isosceles cutin with yellow heart pointed right
}

