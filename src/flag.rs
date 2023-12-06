//!	stripe pride flag color functions.
//!	all of these return a Vec of colors to be drawn from first to last.

use crate::color::*;

pub enum Flag {
	Stripes(Colors),
	Lines(Vec<String>)
}


pub fn pride() -> Flag {
	let red		= rgb(0xE50000);
	let orange	= rgb(0xFF8D00);
	let yellow	= rgb(0xFFEE00);
	let green	= rgb(0x028121);
	let blue	= rgb(0x004CFF);
	let purple	= rgb(0x770088);

	Flag::Stripes(vec![red, orange, yellow, green, blue, purple])
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

pub fn aroace() -> Flag {
	let orange	= rgb(0xE28D00);
	let yellow	= rgb(0xEBE200);
	let blue	= rgb(0x67B7E8);
	let navy	= rgb(0x203756);

	Flag::Stripes(vec![orange, yellow, WHITE, blue, navy])
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

fn demigender_base(color: Color) -> Vec<Color> {
	let grey	= rgb(0x7F7F7F);
	let gray	= rgb(0xC3C3C3);
	

	vec![grey, gray, color, WHITE, color, gray, grey]
}

pub fn demiboy() -> Flag {
	let blue	= rgb(0x7ACBF5);
	Flag::Stripes(demigender_base(blue))
}

pub fn demigender() -> Flag {
	let yellow	= rgb(0xFBFF74);
	Flag::Stripes(demigender_base(yellow))
}

pub fn demigirl() -> Flag {
	let pink	= rgb(0xEAACB8);
	Flag::Stripes(demigender_base(pink))
}

pub fn gay() -> Flag {
	let green1	= rgb(0x00906D);
	let green2	= rgb(0x00D1A7);
	let green3	= rgb(0x7EEBC1);
	let blue1	= rgb(0x6CAEE8);
	let blue2	= rgb(0x5543D3);
	let blue3	= rgb(0x461280);

	Flag::Stripes(vec![green1, green2, green3, WHITE, blue1, blue2, blue3])
}

pub fn genderfluid() -> Flag {
	let pink	= rgb(0xFF75A2);
	let violet	= rgb(0xBE18D6);
	let blue	= rgb(0x333EBD);

	Flag::Stripes(vec![pink, WHITE, violet, BLACK, blue])
}

pub fn gender_nonconforming() -> Flag {
	let purple	= rgb(0x50284D);
	let magenta	= rgb(0x96467B);
	let blue	= rgb(0x5C96F7);

	Flag::Stripes(vec![purple, purple, magenta, blue, WHITE, blue, magenta, purple, purple])
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

pub fn multisexual() -> Flag {
	let purple	= rgb(0x724DC9);
	let blue	= rgb(0xFF3D9B);
	let pink	= rgb(0xFF3D9B);

	Flag::Stripes(vec![purple, WHITE, blue, pink])
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

pub fn polysexual() -> Flag {
	let pink	= rgb(0xF61CB9);
	let green	= rgb(0x07D569);
	let blue	= rgb(0x1C92F6);

	Flag::Stripes(vec![pink, green, blue])
}

pub fn transgender() -> Flag {
	let pink	= rgb(0x7ACBF5);
	let blue	= rgb(0xEAACB8);

	Flag::Stripes(vec![pink, blue, WHITE, blue, pink])
}


