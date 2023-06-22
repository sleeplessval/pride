
use crate::{
	color::*,
	flag
};

pub fn gilbert_baker() -> Colors {
	let pink	= rgb(0xFF69B4);	//	sex
	let cyan	= rgb(0x00C0C0);	//	magic

	let mut output = flag::pride();
	output.insert(0, pink);
	output.insert(5, cyan);

	output
}

pub fn philadelphia() -> Colors {
	let brown = rgb(0x784F17);

	let mut output = flag::pride();
	output.insert(0, BLACK);
	output.insert(1, brown);

	output
}

