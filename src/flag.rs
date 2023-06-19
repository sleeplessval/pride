
use termion::color;

use crate::color::*;
use crate::draw::draw;

pub static BLOCK: &str = "█";

pub fn pride(small: bool) {
	let red		= color::Fg(color::Rgb(0xE5, 0x00, 0x00));
	let orange	= color::Fg(color::Rgb(0xFF, 0x8D, 0x00));
	let yellow	= color::Fg(color::Rgb(0xFF, 0xEE, 0x00));
	let green	= color::Fg(color::Rgb(0x02, 0x81, 0x21));
	let blue	= color::Fg(color::Rgb(0x00, 0x4C, 0xFF));
	let purple	= color::Fg(color::Rgb(0x77, 0x00, 0x88));

	if small {		//	small flag: 18x6
		let width = 18;

		println!(
			"{red}{stripe}\n{orange}{stripe}\n{yellow}{stripe}\n{green}{stripe}\n{blue}{stripe}\n{purple}{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[red, orange, yellow, green, blue, purple]); }
}

pub fn transgender(small: bool) {
	let pink	= color::Fg(color::Rgb(0x7A, 0xCB, 0xF5));
	let blue	= color::Fg(color::Rgb(0xEA, 0xAC, 0xB8));

	if small {
		let width = 15;

		println!(
			"{pink}{stripe}\n{blue}{stripe}\n{WHITE}{stripe}\n{blue}{stripe}\n{pink}{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[pink, blue, WHITE, blue, pink]); }
}

//	everything below here is alphabetical

pub fn agender(small: bool) {
	let gray	= color::Fg(color::Rgb(0xB9, 0xB9, 0xB9));
	let green	= color::Fg(color::Rgb(0xB8, 0xF4, 0x83));

	if small {
		let width = 21;

		println!(
			"{BLACK}{stripe}\n{gray}{stripe}\n{WHITE}{stripe}\n{green}{stripe}\n{WHITE}{stripe}\n{gray}{stripe}\n{BLACK}{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[BLACK, gray, WHITE, green, WHITE, gray, BLACK]); }
}

pub fn aromantic(small: bool) {
	let green	= color::Fg(color::Rgb(0x3B, 0xA7, 0x40));
	let lime	= color::Fg(color::Rgb(0xA8, 0xD4, 0x7A));
	let grey	= color::Fg(color::Rgb(0xAB, 0xAB, 0xAB));

	if small {
		let width = 15;

		println!(
			"{green}{stripe}\n{lime}{stripe}\n{WHITE}{stripe}\n{grey}{stripe}\n{BLACK}{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[green, lime, WHITE, grey, BLACK]); }
}

pub fn asexual(small: bool) {
	let grey	= color::Fg(color::Rgb(0xA4, 0xA4, 0xA4));
	let purple	= color::Fg(color::Rgb(0x81, 0x00, 0x81));

	if small {
		let width = 12;

		println!(
			"{BLACK}{stripe}\n{grey}{stripe}\n{WHITE}{stripe}\n{purple}{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[BLACK, grey, WHITE, purple]); }
}

pub fn bigender(small: bool) {
	let pink	= color::Fg(color::Rgb(0xE6, 0x76, 0xA6));
	let yellow	= color::Fg(color::Rgb(0xF9, 0xF0, 0x4C));
	let purple	= color::Fg(color::Rgb(0xAB, 0x6B, 0xBB));
	let blue	= color::Fg(color::Rgb(0x6D, 0x96, 0xDC));

	if small {
		let width = 15;

		println!(
			"{pink}{stripe}\n{yellow}{stripe}\n{WHITE}{stripe}\n{purple}{stripe}\n{blue}{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[pink, yellow, WHITE, purple, blue]); }
}

pub fn bisexual(small: bool) {
	let magenta	= color::Fg(color::Rgb(0xC4, 0x2A, 0x6F));
	let purple	= color::Fg(color::Rgb(0x91, 0x53, 0x92));
	let blue	= color::Fg(color::Rgb(0x14, 0x37, 0xA2));

	if small {
		let width = 15;

		println!(
			"{magenta}{stripe}\n{stripe}\n{purple}{stripe}\n{blue}{stripe}\n{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[magenta, magenta, purple, blue, blue]); }
}

pub fn genderfluid(small: bool) {
	let pink	= color::Fg(color::Rgb(0xFF, 0x75, 0xA2));
	let violet	= color::Fg(color::Rgb(0xBE, 0x18, 0xD6));
	let blue	= color::Fg(color::Rgb(0x33, 0x3E, 0xBD));

	if small {
		let width = 15;

		println!(
			"{pink}{stripe}\n{WHITE}{stripe}\n{violet}{stripe}\n{BLACK}{stripe}\n{blue}{stripe}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[pink, WHITE, violet, BLACK, blue]); }
}

pub fn genderqueer(small: bool) {
	let purple	= color::Fg(color::Rgb(0xB8, 0x99, 0xDF));
	let green	= color::Fg(color::Rgb(0x6B, 0x8E, 0x3B));

	if small {
		let width = 18;

		println!(
			"{purple}{stripe}\n{stripe}\n{WHITE}{stripe}\n{stripe}\n{green}{stripe}\n{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[purple, WHITE, green]); }
}

pub fn gendervoid(small: bool) {
	let navy	= color::Fg(color::Rgb(0x08, 0x11, 0x4A));
	let gray	= color::Fg(color::Rgb(0x4A, 0x48, 0x4B));

	if small {
		let width = 15;

		println!(
			"{navy}{stripe}\n{gray}{stripe}\n{BLACK}{stripe}\n{gray}{stripe}\n{navy}{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[navy, gray, BLACK, gray, navy]); }
}

pub fn lesbian(small: bool) {
	let red		= color::Fg(color::Rgb(0xD6, 0x28, 0x00));
	let orange	= color::Fg(color::Rgb(0xFF, 0x9B, 0x56));
	let pink	= color::Fg(color::Rgb(0xD4, 0x62, 0xA6));
	let magenta	= color::Fg(color::Rgb(0xA4, 0x00, 0x62));

	if small {
		let width = 15;

		println!(
			"{red}{stripe}\n{orange}{stripe}\n{WHITE}{stripe}\n{pink}{stripe}\n{magenta}{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[red, orange, WHITE, pink, magenta]); }
}

pub fn multigender(small: bool) {
	let blue	= color::Fg(color::Rgb(0x3F, 0x47, 0xCC));
	let ltblue	= color::Fg(color::Rgb(0x01, 0xA4, 0xE9));
	let orange	= color::Fg(color::Rgb(0xFB, 0x7F, 0x27));

	if small {
		let width = 15;

		println!(
			"{blue}{stripe}\n{ltblue}{stripe}\n{orange}{stripe}\n{ltblue}{stripe}\n{blue}{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[blue, ltblue, orange, ltblue, blue]); }
}

pub fn nonbinary(small: bool) {
	let yellow	= color::Fg(color::Rgb(0xFF, 0xF4, 0x33));
	let purple	= color::Fg(color::Rgb(0x9B, 0x59, 0xD0));

	if small {
		let width = 12;

		println!(
			"{yellow}{stripe}\n{WHITE}{stripe}\n{purple}{stripe}\n{BLACK}{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[yellow, WHITE, purple, BLACK]); }
}

pub fn pansexual(small: bool) {
	let magenta	= color::Fg(color::Rgb(0xFF, 0x1B, 0x8D));
	let yellow	= color::Fg(color::Rgb(0xFF, 0xDA, 0x00));
	let cyan	= color::Fg(color::Rgb(0x1B, 0xB3, 0xFF));

	if small {
		let width = 18;

		println!(
			"{magenta}{stripe}\n{stripe}\n{yellow}{stripe}\n{stripe}\n{cyan}{stripe}\n{stripe}{RESET}",
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[magenta, yellow, cyan]); }
}

