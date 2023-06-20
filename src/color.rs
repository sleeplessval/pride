
use termion::color::{ Fg, Rgb, Reset };

pub type Color = Fg<Rgb>;
pub type Colors = Vec<Fg<Rgb>>;

pub static BLACK: Color = Fg(Rgb(0x00, 0x00, 0x00));
pub static WHITE: Color = Fg(Rgb(0xFF, 0xFF, 0xFF));

pub static RESET: Fg<Reset> = Fg(Reset);

pub fn rgb(hex: u32) -> Color {
	let [_, r, g, b] = hex.to_be_bytes();
	
	Fg(Rgb(r, g, b))
}

