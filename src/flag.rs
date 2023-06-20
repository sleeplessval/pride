
use termion::color;

use crate::color::*;

pub static BLOCK: &str = "█";

pub fn pride() -> Colors {
	let red		= color::Fg(color::Rgb(0xE5, 0x00, 0x00));
	let orange	= color::Fg(color::Rgb(0xFF, 0x8D, 0x00));
	let yellow	= color::Fg(color::Rgb(0xFF, 0xEE, 0x00));
	let green	= color::Fg(color::Rgb(0x02, 0x81, 0x21));
	let blue	= color::Fg(color::Rgb(0x00, 0x4C, 0xFF));
	let purple	= color::Fg(color::Rgb(0x77, 0x00, 0x88));

	vec![red, orange, yellow, green, blue, purple]
}

pub fn transgender() -> Colors {
	let pink	= color::Fg(color::Rgb(0x7A, 0xCB, 0xF5));
	let blue	= color::Fg(color::Rgb(0xEA, 0xAC, 0xB8));

	vec![pink, blue, WHITE, blue, pink]
}

//	everything below here is alphabetical

pub fn agender() -> Colors {
	let gray	= color::Fg(color::Rgb(0xB9, 0xB9, 0xB9));
	let green	= color::Fg(color::Rgb(0xB8, 0xF4, 0x83));

	vec![BLACK, gray, WHITE, green, WHITE, gray, BLACK]
}

pub fn aromantic() -> Colors {
	let green	= color::Fg(color::Rgb(0x3B, 0xA7, 0x40));
	let lime	= color::Fg(color::Rgb(0xA8, 0xD4, 0x7A));
	let grey	= color::Fg(color::Rgb(0xAB, 0xAB, 0xAB));

	vec![green, lime, WHITE, grey, BLACK]
}

pub fn asexual() -> Colors {
	let grey	= color::Fg(color::Rgb(0xA4, 0xA4, 0xA4));
	let purple	= color::Fg(color::Rgb(0x81, 0x00, 0x81));

	vec![BLACK, grey, WHITE, purple]
}

pub fn bigender() -> Colors {
	let pink	= color::Fg(color::Rgb(0xE6, 0x76, 0xA6));
	let yellow	= color::Fg(color::Rgb(0xF9, 0xF0, 0x4C));
	let purple	= color::Fg(color::Rgb(0xAB, 0x6B, 0xBB));
	let blue	= color::Fg(color::Rgb(0x6D, 0x96, 0xDC));

	vec![pink, yellow, WHITE, purple, blue]
}

pub fn bisexual() -> Colors {
	let magenta	= color::Fg(color::Rgb(0xC4, 0x2A, 0x6F));
	let purple	= color::Fg(color::Rgb(0x91, 0x53, 0x92));
	let blue	= color::Fg(color::Rgb(0x14, 0x37, 0xA2));

	vec![magenta, magenta, purple, blue, blue]
}

pub fn genderfluid() -> Colors {
	let pink	= color::Fg(color::Rgb(0xFF, 0x75, 0xA2));
	let violet	= color::Fg(color::Rgb(0xBE, 0x18, 0xD6));
	let blue	= color::Fg(color::Rgb(0x33, 0x3E, 0xBD));

	vec![pink, WHITE, violet, BLACK, blue]
}

pub fn genderqueer() -> Colors {
	let purple	= color::Fg(color::Rgb(0xB8, 0x99, 0xDF));
	let green	= color::Fg(color::Rgb(0x6B, 0x8E, 0x3B));

	vec![purple, WHITE, green]
}

pub fn gendervoid() -> Colors {
	let navy	= color::Fg(color::Rgb(0x08, 0x11, 0x4A));
	let gray	= color::Fg(color::Rgb(0x4A, 0x48, 0x4B));

	vec![navy, gray, BLACK, gray, navy]
}

pub fn lesbian() -> Colors {
	let red		= color::Fg(color::Rgb(0xD6, 0x28, 0x00));
	let orange	= color::Fg(color::Rgb(0xFF, 0x9B, 0x56));
	let pink	= color::Fg(color::Rgb(0xD4, 0x62, 0xA6));
	let magenta	= color::Fg(color::Rgb(0xA4, 0x00, 0x62));

	vec![red, orange, WHITE, pink, magenta]
}

pub fn multigender() -> Colors {
	let blue	= color::Fg(color::Rgb(0x3F, 0x47, 0xCC));
	let ltblue	= color::Fg(color::Rgb(0x01, 0xA4, 0xE9));
	let orange	= color::Fg(color::Rgb(0xFB, 0x7F, 0x27));

	vec![blue, ltblue, orange, ltblue, blue]
}

pub fn nonbinary() -> Colors {
	let yellow	= color::Fg(color::Rgb(0xFF, 0xF4, 0x33));
	let purple	= color::Fg(color::Rgb(0x9B, 0x59, 0xD0));

	vec![yellow, WHITE, purple, BLACK]
}

pub fn pansexual() -> Colors {
	let magenta	= color::Fg(color::Rgb(0xFF, 0x1B, 0x8D));
	let yellow	= color::Fg(color::Rgb(0xFF, 0xDA, 0x00));
	let cyan	= color::Fg(color::Rgb(0x1B, 0xB3, 0xFF));

	vec![magenta, yellow, cyan]
}

