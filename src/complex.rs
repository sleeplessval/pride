use std::process::exit;

use crate::color::*;
use crate::draw;
use crate::flag;
use crate::variant;

///	vertically stacking eighths
pub static V_EIGHTH: [char; 7] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇'];
///	horizontally stacking eighths
pub static H_EIGHTH: [char; 7] = ['▏', '▎', '▍', '▌', '▋', '▊', '▉'];

///	shading by intensity
pub static SHADING: [char; 3] = ['░', '▒', '▓'];

///	2/3 slope slant
pub static SLANT_23: [char; 2] = ['🭒', '🭏'];

pub fn progress() -> Colors {
	let red		= bg(0xE50000);
	let orange	= bg(0xFF8D00);
	let yellow	= bg(0xFFEE00);
	let green	= bg(0x028121);
	let blue	= bg(0x004CFF);
	let purple	= bg(0x770088);

	//	we need these colors in both fg & bg; just hold the integers for now
	let black:	u16 = 0;
	let brown:	u16 = 0x784F17;
	let pink:	u16 = 0xEAACB8;
	let white:	u16 = 0xFFFFFF;

	exit(0);
}

//	everything below this point is in alphabetical order

pub fn aroace() {
	let aro = flag::aromantic();
	let ace = flag::asexual();

}

pub fn demiromantic() {
	let green	= rgb(0x3DA542);
	let gray	= rgb(0xD2D2D2);

	//	WHITE×2 / green / gray×2 vert
	//	BLACK triangle cutin
}

pub fn demisexual() {
	let purple	= rgb(0x832FA8);
	let grey	= rgb(0x7B868C);

	//	WHITE×2 / green / grey×2 vert
	//	BLACK triangle cutin
}

pub fn disability() {
	let gray	= bg(0x575757);

	let green	= rgb(0x3AAD7D);
	let blue	= rgb(0x79BFE0);
	let white	= rgb(0xE8E8E8);
	let yellow	= rgb(0xEDDB76);
	let red		= rgb(0xCD7281);

	let stripe = [red, yellow, white, blue, green];

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

