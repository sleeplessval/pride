use std::process::exit;

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
	if args.contains("--version") {
		println!("pride v{VERSION}");
		return;
	}

	//	get small flag
	let small = args.contains(["-s", "--small"]);

	let subcommand = args.subcommand().unwrap();

	//	get color vec from matched flag
	let flag: Flag = match subcommand.as_deref() {
		Some("pride" | "rainbow")
		| None
			=>	{
				let variant = args.subcommand().unwrap_or(None);
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

		Some("transgender" | "trans")
			=>	flag::transgender(),


		Some("agender")
			=>	flag::agender(),

		Some("aromantic" | "aro")
			=>	flag::aromantic(),

		Some("asexual" | "ace")
			=>	flag::asexual(),

		Some("aroace" | "aromantic-asexual")
			=>	complex::aroace(small),

		Some("bigender")
			=>	flag::bigender(),

		Some("bisexual" | "bi")
			=>	flag::bisexual(),

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

		Some("intersex")
			=>	complex::intersex(),

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

		Some("multigender")
			=>	flag::multigender(),

		Some("nonbinary" | "nb")
			=>	flag::nonbinary(),

		Some("pansexual" | "pan")
			=>	flag::pansexual(),

//		Some("poly" | "polyamorous" | "polyamory")
//			=>	complex::polyamorous(),

		Some("progress")
			=>	complex::progress(small),

		_ => { help::help_text(); exit(1) }
	};

	flag.draw(!small);

}

