use std::process::exit;

pub fn unmatched_flag(target: String) {
	eprintln!("pride: no flag {target}");
	exit(1);
}


pub fn size_missing() {
	eprintln!("pride: size flag requires a value");
	exit(2);
}

pub fn size_error(value: &str) {
	eprintln!("pride: size '{value}' is invalid");
	exit(2);
}


pub fn too_small(width: u16, height: u16) {
	eprintln!("pride: this flag must be bigger than {width}x{height}");
	exit(3);
}

