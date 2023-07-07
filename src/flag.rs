
use crate::color::*;
use crate::draw::Flag;

pub fn pride() -> Flag {
	let red		= rgb(0xE50000);
	let orange	= rgb(0xFF8D00);
	let yellow	= rgb(0xFFEE00);
	let green	= rgb(0x028121);
	let blue	= rgb(0x004CFF);
	let purple	= rgb(0x770088);

	Flag::Stripes(vec![red, orange, yellow, green, blue, purple])
}

pub fn transgender() -> Flag {
	let pink	= rgb(0x7ACBF5);
	let blue	= rgb(0xEAACB8);

	Flag::Stripes(vec![pink, blue, WHITE, blue, pink])
}

//	everything below here is alphabetical

pub fn agender() -> Flag {
	let gray	= rgb(0xB9B9B9);
	let green	= rgb(0xB8F483);

	Flag::Stripes(vec![BLACK, gray, WHITE, green, WHITE, gray, BLACK])
}

pub fn aromantic() -> Flag {
	let green	= rgb(0x3BA740);
	let lime	= rgb(0xA8D47A);
	let grey	= rgb(0xABABAB);

	Flag::Stripes(vec![green, lime, WHITE, grey, BLACK])
}

pub fn asexual() -> Flag {
	let grey	= rgb(0xA4A4A4);
	let purple	= rgb(0x810081);

	Flag::Stripes(vec![BLACK, grey, WHITE, purple])
}

pub fn bigender() -> Flag {
	let pink	= rgb(0xE676A6);
	let yellow	= rgb(0xF9F04C);
	let purple	= rgb(0xAB6BBB);
	let blue	= rgb(0x6D96DC);

	Flag::Stripes(vec![pink, yellow, WHITE, purple, blue])
}

pub fn bisexual() -> Flag {
	let magenta	= rgb(0xC42A6F);
	let purple	= rgb(0x915392);
	let blue	= rgb(0x1437A2);

	Flag::Stripes(vec![magenta, magenta, purple, blue, blue])
}

pub fn genderfluid() -> Flag {
	let pink	= rgb(0xFF75A2);
	let violet	= rgb(0xBE18D6);
	let blue	= rgb(0x333EBD);

	Flag::Stripes(vec![pink, WHITE, violet, BLACK, blue])
}

pub fn genderqueer() -> Flag {
	let purple	= rgb(0xB899DF);
	let green	= rgb(0x6B8E3B);

	Flag::Stripes(vec![purple, WHITE, green])
}

pub fn gendervoid() -> Flag {
	let navy	= rgb(0x08114A);
	let gray	= rgb(0x4A484B);

	Flag::Stripes(vec![navy, gray, BLACK, gray, navy])
}

pub fn lesbian() -> Flag {
	let red		= rgb(0xD62800);
	let orange	= rgb(0xFF9B56);
	let pink	= rgb(0xD462A6);
	let magenta	= rgb(0xA40062);

	Flag::Stripes(vec![red, orange, WHITE, pink, magenta])
}

pub fn multigender() -> Flag {
	let blue	= rgb(0x3F47CC);
	let ltblue	= rgb(0x01A4E9);
	let orange	= rgb(0xFB7F27);

	Flag::Stripes(vec![blue, ltblue, orange, ltblue, blue])
}

pub fn nonbinary() -> Flag {
	let yellow	= rgb(0xFFF433);
	let purple	= rgb(0x9B59D0);

	Flag::Stripes(vec![yellow, WHITE, purple, BLACK])
}

pub fn pansexual() -> Flag {
	let magenta	= rgb(0xFF1B8D);
	let yellow	= rgb(0xFFDA00);
	let cyan	= rgb(0x1BB3FF);

	Flag::Stripes(vec![magenta, yellow, cyan])
}

