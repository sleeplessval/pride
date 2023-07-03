
use termion::color::{ Bg, Fg, Rgb, Reset };

pub type Color = Fg<Rgb>;
pub type Colors = Vec<Fg<Rgb>>;

pub static BLACK: Color = Fg(Rgb(0x00, 0x00, 0x00));
pub static WHITE: Color = Fg(Rgb(0xFF, 0xFF, 0xFF));

pub static RESET: Fg<Reset> = Fg(Reset);
pub static RESET_BG: Bg<Reset> = Bg(Reset);

///	produces a termion foreground color from the provided integer
pub fn rgb(hex: u32) -> Color {
	let [_, r, g, b] = hex.to_be_bytes();
	
	Fg(Rgb(r, g, b))
}

///	produces a termion background color from the provided integer
pub fn bg(hex: u32) -> Bg<Rgb> {
	let [_, r, g, b] = hex.to_be_bytes();

	Bg(Rgb(r, g, b))
}

