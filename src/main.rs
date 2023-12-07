//!	main method module

use std::{
	io::{ stdout, IsTerminal },
	process::exit
};

use pico_args::Arguments;

mod color;
mod complex;
mod draw;
mod flag;
mod help;
mod util;
mod variant;

use crate::flag::Flag;

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
	if args.contains(["-v", "--version"]) {
		println!("pride v{VERSION}");
		return;
	}

	if !stdout().is_terminal() {
		println!("pride: output must be a terminal");
		exit(2);
	}

	//	get small flag
	let small = args.contains(["-s", "--small"]);

	let subcommand	= args.subcommand().unwrap();
	let variant		= args.subcommand().unwrap_or(None);

	//	get color vec from matched flag
	let flag: Flag = match subcommand.as_deref() {
		Some("pride" | "rainbow")
		| None
			=>	{
				match variant.as_deref() {
					Some("8-color" | "gilbert-baker" | "sex-and-magic")
						=>	variant::gilbert_baker(),
					Some("philadelphia")
						=>	variant::philadelphia(),
					Some("progress")
						=>	complex::progress(small),
					_
						=>	flag::pride()
				}
			},

		Some("progress")
			=>	complex::progress(small),


		Some("agender")
			=>	flag::agender(),

		Some("aromantic" | "aro")
			=>	flag::aromantic(),

		Some("asexual" | "ace")
			=>	flag::asexual(),

		Some("aroace" | "aromantic-asexual")
			=>	{
				match variant.as_deref() {
					Some("halves" | "side-by-side" | "sbs")
						=>	complex::aroace(small),
					_
						=>	flag::aroace()
				}
			},

		Some("bigender")
			=>	flag::bigender(),

		Some("bisexual" | "bi")
			=>	flag::bisexual(),


		Some("demiboy")
			=>	flag::demiboy(),
		Some("demigender")
			=>	flag::demigender(),
		Some("demigirl")
			=>	flag::demigirl(),

		Some("demiromantic")
			=>	complex::demiromantic(small),

		Some("demisexual")
			=>	complex::demisexual(small),

//		Some("disability")
//			=>	complex::disability();

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

//		Some("intersex")
//			=>	complex::intersex(),

		Some("lesbian")
			=>	{
				let variant = args.subcommand().unwrap_or(None);
				match variant.as_deref() {
					Some("7-color")
						=>	variant::lesbian_7(),
					_
						=>	flag::lesbian()
				}
			}

		Some("multisexual" | "m-spec" | "mspec")
			=>	flag::multisexual(),

		Some("multigender")
			=>	flag::multigender(),

		Some("nonbinary" | "nb")
			=>	flag::nonbinary(),

		Some("pansexual" | "pan")
			=>	flag::pansexual(),

//		Some("polyamory" | "polyamorous" | "poly")
//			=>	complex::polyamory(small),

		Some("polysexual")
			=>	flag::polysexual(),

		Some("transgender" | "trans")
			=>	flag::transgender(),


		_ => { help::help_text(); exit(1) }
	};

	//	draw flag
	flag.draw(!small);

}

