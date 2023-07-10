//!	flags that require more complex rendering than just scaling colored stripes

use termion::{
	terminal_size,

	color::{ Bg, Rgb }
};

use crate::{
	color::*,
	draw,
	flag::{ self, Flag },
	util::{ ansi_len, ansi_substr }
};

///	vertically stacking eighths
pub static V_EIGHTH: [char; 7] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇'];
///	horizontally stacking eighths
pub static H_EIGHTH: [char; 7] = ['▏', '▎', '▍', '▌', '▋', '▊', '▉'];

///	shading by intensity
pub static SHADING: [char; 3] = ['░', '▒', '▓'];

///	2/1 slope triangle cut in
pub static TRIANGLE_21: [char; 3] = ['', '🭬', ''];

///	2/3 slope slant
pub static SLANT_23: [char; 2] = ['🭒', '🭏'];

pub fn progress(small: bool) -> Flag {
	let red		= bg(0xE50000);
	let orange	= bg(0xFF8D00);
	let yellow	= bg(0xFFEE00);
	let green	= bg(0x028121);
	let blue	= bg(0x004CFF);
	let purple	= bg(0x770088);

	//	we need these colors in both fg & bg; just hold the integers for now
	let black:	u32 = 0;
	let brown:	u32 = 0x784F17;
	let ltblue:	u32 = 0xEAACB8;
	let pink:	u32 = 0x7ACBF5;
	let white:	u32 = 0xFFFFFF;

	let (width, height) = if small { (18, 6) } else { terminal_size().unwrap() };

	//	create color slices and line buffer
	let stripes = [red, orange, yellow, green, blue, purple];
	let chevrons = [white, ltblue, pink, brown, black];
	let mut lines: Vec<String> = Vec::new();

	//	set up stripe index
	let mut index = 0;

	/*	ok, coming up with procedure:
	 *	- can't rely on bg_stripes; line count, threshold, etc., will need to happen here
	 *	- line count will always be the largest multiple of 6 smaller than height; c = h - (h % 6)
	 *	- chevrons may have larger widths: TODO calc
	 *	- depth will be funky; line depth will need to use "full" depth; (Df - Dl) / Wc = Ic (TODO verify?)
	 *	- switch stripe index on *absolute* line number rather than n
	 *	- chevron building will be BLOCK.repeat(width) + TRIANGLE_21[0] (fg Ic, bg Ic + 1)
	 *		- chevrons[len - 1] will need unique handling to draw stripes
	 */

	//	set up constraints
	let linecount = height - (height % 6);	//	largest multiple of 6 smaller than height
	let full_depth = width / 3;
	let chevron_width = (full_depth / 6) - 1;
	let direction_thresh = linecount / 2;
	let corner = linecount % 2 == 1;
	
	let thresh = linecount / 6;				//	stripe threshold; no bg_stripes call!
	let mut line_no = 0;					//	absolute line number; n is relative

	//	chevron helper
	let build_chevron = | separator: char | -> String {
		let mut output = format!(
			"{fg}{bg}{stripe}{separator}",
			fg = rgb(chevrons[0]),
			bg = bg(chevrons[1]),
			stripe = draw::BLOCK.repeat( usize::max(chevron_width as usize * 2, 1) )
		);
		let stripe = draw::BLOCK.repeat(chevron_width as usize);
		for i in 1..4 {
			output += &format!(
				"{fg}{bg}{stripe}{separator}",
				fg = rgb(chevrons[i]),
				bg = bg(chevrons[i + 1])
			);
		}
		output += &format!(
			"{fg}{stripe}",
			fg = rgb(chevrons[4])
		);

		output
	};

	//	piecewise functions: ascent -> peak -> descent
	let mut base = build_chevron(TRIANGLE_21[0]);
	let base_length = base.len();
	let display_length = ansi_len(&base) + 1;	//	chevron width will always stay the same; add 1 for the last separator
	for n in 0..direction_thresh {
		//	advance stripe color at stripe threshold by line number
		if line_no != 0 && line_no % thresh == 0 { index += 1; }

		//	grab our substring constraints
		let start = (direction_thresh - n) as usize - 1;
		let diff = display_length - start;

		//	take substring of chevron line...
		let mut line = ansi_substr(&base, start as usize, base_length);
		//	... and add the colored stripe
		line += &format!(
			"{stripe}{separator}{line}",
			stripe = stripes[index],
			separator = TRIANGLE_21[0],
			line = " ".repeat(width as usize - diff)
		);

		lines.push(line);
		line_no += 1;
	}
	if corner {
		if line_no % thresh == 0 { index += 1; }

		let base = build_chevron(TRIANGLE_21[1]);
		let mut line = ansi_substr(&base, 0, base_length);
		line += &format!(
			"{stripe}{separator}{line}",
			stripe = stripes[index],
			separator = TRIANGLE_21[1],
			line = " ".repeat(width as usize - display_length)
		);

		lines.push(line);
		line_no += 1;
	}
	base = build_chevron(TRIANGLE_21[2]);
	for n in 0..direction_thresh {
		if line_no % thresh == 0 { index += 1; }
		if index > 5 { break; }

		let start = n as usize;
		let diff = display_length - start;

		let mut line = ansi_substr(&base, start, base_length);
		line += &format!(
			"{stripe}{separator}{line}",
			stripe = stripes[index],
			separator = TRIANGLE_21[2],
			line = " ".repeat(width as usize - diff)
		);

		lines.push(line);
		line_no += 1;
	}

	Flag::Lines(lines)
}

//	everything below this point is in alphabetical order

pub fn aroace(small: bool) -> Flag {
	//	pull colors from aro & ace stripe flags
	let Flag::Stripes(aro) = flag::aromantic() else { panic!() };
	let Flag::Stripes(ace) = flag::asexual() else { panic!() };

	let (width, height) = if small { (60, 20) } else { terminal_size().unwrap() };

	let mut lines: Vec<String> = Vec::new();

	//	set up constraints
	let linecount = height - (height % 20);
	let aro_thresh = linecount / 5;	//	threshold for aromantic colors
	let ace_thresh = linecount / 4;	//	threshold for asexual colors
	let stripe = draw::BLOCK.repeat((width / 2) as usize);

	let mut aro_index = 0;
	let mut ace_index = 0;
	for n in 0..linecount {
		//	switch colors on thresholds
		if n != 0 {
			if n % aro_thresh == 0 { aro_index += 1; }
			if n % ace_thresh == 0 { ace_index += 1; }
		}
		let line = format!("{}{stripe}{}{stripe}", aro[aro_index], ace[ace_index]);
		lines.push(line);
	}

	Flag::Lines(lines)
}

fn demi_orientation_render(middle: Bg<Rgb>, bottom: Bg<Rgb>, width: u16, height: u16) -> Vec<String> {
	let white	= bg(0xFFFFFF);

	let stripes = vec![white, white, middle, bottom, bottom];

	//	initial stripe output buffer
	let mut lines: Vec<String> = draw::bg_stripes(stripes, width, height);

	//	assemble triangle cut-in
	let linecount = lines.len();
	let depth = linecount / 2;
	let corner = linecount % 2 == 1;
	//	piecewise functions: ascending -> peak -> descending
	for n in 0..depth {
		let line = lines[n].clone();

		let replacement = format!("{BLACK}{}{}", draw::BLOCK.repeat(n), TRIANGLE_21[0]);
		lines[n] = line.replacen(&" ".repeat(n + 1), &replacement, 1);
	}
	if corner {
		let line = lines[depth].clone();

		let replacement = format!("{BLACK}{}{}", draw::BLOCK.repeat(depth), TRIANGLE_21[1]);
		lines[depth] = line.replacen(&" ".repeat(depth + 1), &replacement, 1);
	}
	let start = depth + (linecount % 2);
	for n in 0..depth {
		let line = lines[start + n].clone();

		let size = depth - n - 1;
		let replacement = format!("{BLACK}{}{}", draw::BLOCK.repeat(size), TRIANGLE_21[2]);
		lines[start + n] = line.replacen(&" ".repeat(size + 1), &replacement, 1);
	}

	lines
}

pub fn demiromantic(small: bool) -> Flag {
	let green	= bg(0x3DA542);
	let gray	= bg(0xD2D2D2);

	let (width, height) = if small { (15, 5) } else { terminal_size().unwrap() };
	let lines = demi_orientation_render(green, gray, width, height);

	Flag::Lines(lines)
}

pub fn demisexual(small: bool) -> Flag {
	let purple	= bg(0x832FA8);
	let grey	= bg(0x7B868C);

	let (width, height) = if small { (15, 5) } else { terminal_size().unwrap() };
	let lines = demi_orientation_render(purple, grey, width, height);

	Flag::Lines(lines)
}

pub fn disability() {
	let gray	= bg(0x575757);

	let green:	u32 = 0x3AAD7D;
	let blue:	u32 = 0x79BFE0;
	let white:	u32 = 0xE8E8E8;
	let yellow:	u32 = 0xEDDB76;
	let red:	u32 = 0xCD7281;

	let stripes = [red, yellow, white, blue, green];

	// 2/3 slant stripes with gray background
	
}

pub fn intersex() -> Flag {
	let yellow	= bg(0xFFDA00);
	let purple	= rgb(0x7A00AC);

	let block = " ";
	let stripe = block.repeat(9);
	let part = block.repeat(4);

	let lines = vec![
		format!("{yellow}{stripe}"),
		format!("{part}{purple}O{part}"),
		format!("{stripe}")
	];

	Flag::Lines(lines)
}

pub fn polyamorous() {
	let blue	= rgb(0x019FE3);
	let magenta	= rgb(0xE50051);
	let purple	= rgb(0x340C46);
	let yellow	= rgb(0x00FCBF);

	//	blue / magenta / purple vert
	//	WHITE isosceles cutin with yellow heart pointed right
}

