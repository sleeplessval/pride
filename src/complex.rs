use std::process::exit;

use crate::color::*;
use crate::draw;
use crate::flag;
use crate::variant;

pub fn progress() {
	let philadelphia = variant::philadelphia();
	let trans = flag::transgender();

	
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

pub fn intersex() -> Colors {
	let yellow	= rgb(0xFFDA00);
	let purple	= rgb(0x7A00AC);

	let stripe = draw::BLOCK.repeat(9);
	let part = draw::BLOCK.repeat(4);

	println!(
		"{yellow}{stripe}\n{part}{purple}{}O{}{yellow}{part}\n{stripe}{RESET}",
		yellow.0.bg_string(),
		RESET.0.bg_str()
	);
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

