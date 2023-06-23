
use crate::color::*;

pub fn pride() -> Colors {
	let red		= rgb(0xE50000);
	let orange	= rgb(0xFF8D00);
	let yellow	= rgb(0xFFEE00);
	let green	= rgb(0x028121);
	let blue	= rgb(0x004CFF);
	let purple	= rgb(0x770088);

	vec![red, orange, yellow, green, blue, purple]
}

pub fn transgender() -> Colors {
	let pink	= rgb(0x7ACBF5);
	let blue	= rgb(0xEAACB8);

	vec![pink, blue, WHITE, blue, pink]
}

//	everything below here is alphabetical

pub fn agender() -> Colors {
	let gray	= rgb(0xB9B9B9);
	let green	= rgb(0xB8F483);

	vec![BLACK, gray, WHITE, green, WHITE, gray, BLACK]
}

pub fn aromantic() -> Colors {
	let green	= rgb(0x3BA740);
	let lime	= rgb(0xA8D47A);
	let grey	= rgb(0xABABAB);

	vec![green, lime, WHITE, grey, BLACK]
}

pub fn asexual() -> Colors {
	let grey	= rgb(0xA4A4A4);
	let purple	= rgb(0x810081);

	vec![BLACK, grey, WHITE, purple]
}

pub fn bigender() -> Colors {
	let pink	= rgb(0xE676A6);
	let yellow	= rgb(0xF9F04C);
	let purple	= rgb(0xAB6BBB);
	let blue	= rgb(0x6D96DC);

	vec![pink, yellow, WHITE, purple, blue]
}

pub fn bisexual() -> Colors {
	let magenta	= rgb(0xC42A6F);
	let purple	= rgb(0x915392);
	let blue	= rgb(0x1437A2);

	vec![magenta, magenta, purple, blue, blue]
}

pub fn genderfluid() -> Colors {
	let pink	= rgb(0xFF75A2);
	let violet	= rgb(0xBE18D6);
	let blue	= rgb(0x333EBD);

	vec![pink, WHITE, violet, BLACK, blue]
}

pub fn genderqueer() -> Colors {
	let purple	= rgb(0xB899DF);
	let green	= rgb(0x6B8E3B);

	vec![purple, WHITE, green]
}

pub fn gendervoid() -> Colors {
	let navy	= rgb(0x08114A);
	let gray	= rgb(0x4A484B);

	vec![navy, gray, BLACK, gray, navy]
}

pub fn lesbian() -> Colors {
	let red		= rgb(0xD62800);
	let orange	= rgb(0xFF9B56);
	let pink	= rgb(0xD462A6);
	let magenta	= rgb(0xA40062);

	vec![red, orange, WHITE, pink, magenta]
}

pub fn multigender() -> Colors {
	let blue	= rgb(0x3F47CC);
	let ltblue	= rgb(0x01A4E9);
	let orange	= rgb(0xFB7F27);

	vec![blue, ltblue, orange, ltblue, blue]
}

pub fn nonbinary() -> Colors {
	let yellow	= rgb(0xFFF433);
	let purple	= rgb(0x9B59D0);

	vec![yellow, WHITE, purple, BLACK]
}

pub fn pansexual() -> Colors {
	let magenta	= rgb(0xFF1B8D);
	let yellow	= rgb(0xFFDA00);
	let cyan	= rgb(0x1BB3FF);

	vec![magenta, yellow, cyan]
}

