use std::process::exit;

use pico_args::Arguments;

mod color;
mod draw;
mod flag;
mod help;
mod variant;

use crate::color::Colors;

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	//	collect args
	let mut args = Arguments::from_env();

	//	handle help flag
	if args.contains(["-h", "--help"]) {
		let target = args.subcommand().unwrap();
		if target.is_some() { help::flag_help(&target.unwrap()); }
		else { help::help_text(); }
		return;
	}

	//	handle list flag
	if args.contains(["-l", "--list"]) {
		help::list_text();
		return;
	}

	//	handle version flag
	if args.contains("--version") {
		println!("pride v{VERSION}");
		return;
	}

	//	get small flag
	let small = args.contains(["-s", "--small"]);

	let subcommand = args.subcommand().unwrap();

	//	get color vec from matched flag
	let colors: Colors = match subcommand.as_deref() {
		Some("pride" | "rainbow")
		| None
			=>	{
				let variant = args.subcommand().unwrap_or(None);
				match variant.as_deref() {
					Some("8-color" | "gilbert-baker" | "sex-and-magic")
						=>	variant::gilbert_baker(),
					Some("philadelphia")
						=>	variant::philadelphia(),
					_
						=>	flag::pride()
				}
			},

		Some("transgender" | "trans")
			=>	flag::transgender(),


		Some("agender")
			=>	flag::agender(),

		Some("aromantic" | "aro")
			=>	flag::aromantic(),

		Some("asexual" | "ace")
			=>	flag::asexual(),

		Some("bigender")
			=>	flag::bigender(),

		Some("bisexual" | "bi")
			=>	flag::bisexual(),

		Some("gay" | "mlm")
			=>	flag::gay(),

		Some("genderfluid")
			=>	flag::genderfluid(),

		Some("gender-nonconforming" | "gnc" | "gendernonconforming")
			=>	flag::gender_nonconforming(),

		Some("genderqueer")
			=>	flag::genderqueer(),

		Some("gendervoid")
			=>	flag::gendervoid(),

		Some("lesbian")
			=>	flag::lesbian(),

		Some("multigender")
			=>	flag::multigender(),

		Some("nonbinary" | "nb")
			=>	flag::nonbinary(),

		Some("pansexual" | "pan")
			=>	flag::pansexual(),

		_ => { help::help_text(); exit(1) }	//	(or die)
	};

	if small { draw::small(colors); }
	else { draw::full(colors); }

}

