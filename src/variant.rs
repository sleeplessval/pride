//!	variant pride flags
//!	these aren't in the flag module for organizational reasons.

use crate::{
	color::*,
	flag
};

pub fn gilbert_baker() -> Colors {
	let pink	= rgb(0xFF69B4);	//	sex
	let red		= rgb(0xFF0000);	//	life
	let orange	= rgb(0xFF8F00);	//	healing
	let yellow	= rgb(0xFFFF00);	//	sunlight
	let green	= rgb(0x008F00);	//	nature
	let cyan	= rgb(0x00C0C0);	//	magic
	let indigo	= rgb(0x3E0099);	//	serenity
	let purple	= rgb(0x8F008F);	//	spirit

	vec![pink, red, orange, yellow, green, cyan, indigo, purple]
}

pub fn philadelphia() -> Colors {
	let brown = rgb(0x784F17);

	let mut output = flag::pride();
	output.insert(0, BLACK);
	output.insert(1, brown);

	output
}

pub fn lesbian_7() -> Colors {
	let orange1	= rgb(0xD52D00);	//	gender non-conformity
	let orange2	= rgb(0xEF7627);	//	independence
	let orange3	= rgb(0xFF9A56);	//	community
	//	white						//	unique relationships with womanhood
	let pink1	= rgb(0xD162A4);	//	serenity and peace
	let pink2	= rgb(0xB55690);	//	love and sex
	let pink3	= rgb(0xA30262);	//	femininity

	vec![orange1, orange2, orange3, WHITE, pink1, pink2, pink3]
}

