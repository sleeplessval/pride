
use crate::{
	color::*,
	flag::{ self, Flag }
};

pub fn gilbert_baker() -> Flag {
	let pink	= rgb(0xFF69B4);	//	sex
	let red		= rgb(0xFF0000);	//	life
	let orange	= rgb(0xFF8F00);	//	healing
	let yellow	= rgb(0xFFFF00);	//	sunlight
	let green	= rgb(0x008F00);	//	nature
	let cyan	= rgb(0x00C0C0);	//	magic
	let indigo	= rgb(0x3E0099);	//	serenity
	let purple	= rgb(0x8F008F);	//	spirit

	Flag::Stripes(vec![pink, red, orange, yellow, green, cyan, indigo, purple])
}

pub fn philadelphia() -> Flag {
	let brown = rgb(0x784F17);

	let base = flag::pride();
	let mut colors = match base {
		Flag::Stripes(inner)
			=>	inner,
		_
			=>	{ panic!("impossible enum variant"); }
	};
	colors.insert(0, BLACK);
	colors.insert(1, brown);

	Flag::Stripes(colors)
}

