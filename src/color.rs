
use termion::color::{ Fg, Rgb, Reset };

pub static BLACK: Fg<Rgb> = Fg(Rgb(0x00, 0x00, 0x00));
pub static WHITE: Fg<Rgb> = Fg(Rgb(0xFF, 0xFF, 0xFF));

pub static RESET: Fg<Reset> = Fg(Reset);

