
use termion::color;

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
			"{red}{stripe}\n{orange}{stripe}\n{yellow}{stripe}\n{green}{stripe}\n{blue}{stripe}\n{purple}{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[red, orange, yellow, green, blue, purple]); }
}

pub fn transgender(small: bool) {
	let pink	= color::Fg(color::Rgb(0x7A, 0xCB, 0xF5));
	let blue	= color::Fg(color::Rgb(0xEA, 0xAC, 0xB8));
	let white	= color::Fg(color::Rgb(0xFF, 0xFF, 0xFF));

	if small {
		let width = 15;

		println!(
			"{pink}{stripe}\n{blue}{stripe}\n{white}{stripe}\n{blue}{stripe}\n{pink}{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[pink, blue, white, blue, pink]); }
}

//	everything below here is alphabetical

pub fn aromantic(small: bool) {
	let green	= color::Fg(color::Rgb(0x3B, 0xA7, 0x40));
	let lime	= color::Fg(color::Rgb(0xA8, 0xD4, 0x7A));
	let white	= color::Fg(color::Rgb(0xFF, 0xFF, 0xFF));
	let grey	= color::Fg(color::Rgb(0xAB, 0xAB, 0xAB));
	let black	= color::Fg(color::Rgb(0x00, 0x00, 0x00));

	if small {
		let width = 15;

		println!(
			"{green}{stripe}\n{lime}{stripe}\n{white}{stripe}\n{grey}{stripe}\n{black}{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[green, lime, white, grey, black]); }
}

pub fn asexual(small: bool) {
	let black	= color::Fg(color::Rgb(0x00, 0x00, 0x00));
	let grey	= color::Fg(color::Rgb(0xA4, 0xA4, 0xA4));
	let white	= color::Fg(color::Rgb(0xFF, 0xFF, 0xFF));
	let purple	= color::Fg(color::Rgb(0x81, 0x00, 0x81));

	if small {
		let width = 12;

		println!(
			"{black}{stripe}\n{grey}{stripe}\n{white}{stripe}\n{purple}{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[black, grey, white, purple]); }
}

pub fn bigender(small: bool) {
	let pink	= color::Fg(color::Rgb(0xE6, 0x76, 0xA6));
	let yellow	= color::Fg(color::Rgb(0xF9, 0xF0, 0x4C));
	let white	= color::Fg(color::Rgb(0xFF, 0xFF, 0xFF));
	let purple	= color::Fg(color::Rgb(0xAB, 0x6B, 0xBB));
	let blue	= color::Fg(color::Rgb(0x6D, 0x96, 0xDC));

	if small {
		let width = 15;

		println!(
			"{pink}{stripe}\n{yellow}{stripe}\n{white}{stripe}\n{purple}{stripe}\n{blue}{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[pink, yellow, white, purple, blue]); }
}

pub fn bisexual(small: bool) {
	let magenta	= color::Fg(color::Rgb(0xC4, 0x2A, 0x6F));
	let purple	= color::Fg(color::Rgb(0x91, 0x53, 0x92));
	let blue	= color::Fg(color::Rgb(0x14, 0x37, 0xA2));

	if small {
		let width = 15;

		println!(
			"{magenta}{stripe}\n{stripe}\n{purple}{stripe}\n{blue}{stripe}\n{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[magenta, magenta, purple, blue, blue]); }
}

pub fn gendervoid(small: bool) {
	let navy	= color::Fg(color::Rgb(0x08, 0x11, 0x4A));
	let gray	= color::Fg(color::Rgb(0x4A, 0x48, 0x4B));
	let black	= color::Fg(color::Rgb(0x00, 0x00, 0x00));

	if small {
		let width = 15;

		println!(
			"{navy}{stripe}\n{gray}{stripe}\n{black}{stripe}\n{gray}{stripe}\n{navy}{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[navy, gray, black, gray, navy]); }
}

pub fn lesbian(small: bool) {
	let red		= color::Fg(color::Rgb(0xD6, 0x28, 0x00));
	let orange	= color::Fg(color::Rgb(0xFF, 0x9B, 0x56));
	let white	= color::Fg(color::Rgb(0xFF, 0xFF, 0xFF));
	let pink	= color::Fg(color::Rgb(0xD4, 0x62, 0xA6));
	let magenta	= color::Fg(color::Rgb(0xA4, 0x00, 0x62));

	if small {
		let width = 15;

		println!(
			"{red}{stripe}\n{orange}{stripe}\n{white}{stripe}\n{pink}{stripe}\n{magenta}{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[red, orange, white, pink, magenta]); }
}

pub fn multigender(small: bool) {
	let blue	= color::Fg(color::Rgb(0x3F, 0x47, 0xCC));
	let ltblue	= color::Fg(color::Rgb(0x01, 0xA4, 0xE9));
	let orange	= color::Fg(color::Rgb(0xFB, 0x7F, 0x27));

	if small {
		let width = 15;

		println!(
			"{blue}{stripe}\n{ltblue}{stripe}\n{orange}{stripe}\n{ltblue}{stripe}\n{blue}{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[blue, ltblue, orange, ltblue, blue]); }
}

pub fn nonbinary(small: bool) {
	let yellow	= color::Fg(color::Rgb(0xFF, 0xF4, 0x33));
	let white	= color::Fg(color::Rgb(0xFF, 0xFF, 0xFF));
	let purple	= color::Fg(color::Rgb(0x9B, 0x59, 0xD0));
	let black	= color::Fg(color::Rgb(0x00, 0x00, 0x00));

	if small {
		let width = 12;

		println!(
			"{yellow}{stripe}\n{white}{stripe}\n{purple}{stripe}\n{black}{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[yellow, white, purple, black]); }
}

pub fn pansexual(small: bool) {
	let magenta	= color::Fg(color::Rgb(0xFF, 0x1B, 0x8D));
	let yellow	= color::Fg(color::Rgb(0xFF, 0xDA, 0x00));
	let cyan	= color::Fg(color::Rgb(0x1B, 0xB3, 0xFF));

	if small {
		let width = 18;

		println!(
			"{magenta}{stripe}\n{stripe}\n{yellow}{stripe}\n{stripe}\n{cyan}{stripe}\n{stripe}{reset}",
			reset = color::Fg(color::Reset),
			stripe = BLOCK.repeat(width)
		);
	} else { draw(&[magenta, yellow, cyan]); }
}

