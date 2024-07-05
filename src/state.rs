use std::io::{ stdout, IsTerminal };

use pico_args::{ Arguments, Error };
use termion::terminal_size;

use crate::{ FLAG_SIZE, error };

#[derive(PartialEq)]
pub enum Size {
	Full,
	Small,
	Set(u16, u16),
	Wide(u16)
}

impl Size {
	pub fn from(value: &str) -> Size {
		if value == "small" { return Size::Small; }
		let split: Vec<&str> = value.split('x').collect();
		let len = split.len();
		if len == 2 {
			if let (Ok(width), Ok(height)) = (str::parse::<u16>(split.get(0).unwrap()), str::parse::<u16>(split.get(1).unwrap())) {
				return Size::Set(width, height);
			}
		} else if len == 1 {
			if let Ok(width) = str::parse::<u16>(split.get(0).unwrap()) {
				return Size::Wide(width);
			}
		}
		error::size_error(value);
		panic!();
	}
	pub fn get(&self, width: u16, height: u16) -> (u16, u16) {
		match self {
			Size::Full					=> terminal_size().unwrap(),
			Size::Set(width, height)	=> (width.clone(), height.clone()),
			Size::Small					=> (width, height),
			Size::Wide(width)			=> (width.clone(), height)
		}
	}
}

pub struct State {
	pub size: Size,
	pub is_terminal: bool,
	pub flag: Option<String>,
	pub variant: Option<String>
}

impl State {
	pub fn new(args: &mut Arguments) -> State {
		let is_terminal = stdout().is_terminal();

		let size = match args.value_from_str::<[&str;2], String>(FLAG_SIZE).as_deref() {
			Ok(value)						=>	Size::from(value),
			Err(Error::MissingOption(_)) |
			Err(Error::MissingArgument)		=>	if is_terminal { Size::Full } else { Size::Small },
			_								=>	{ error::size_missing(); panic!() }
		};

		let flag = args.subcommand().unwrap();
		let variant = args.subcommand().unwrap();

		State { size, is_terminal, flag, variant }
	}
}

