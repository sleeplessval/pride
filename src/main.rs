//!	main method module
use std::env::var;

use pico_args::Arguments;

mod color;
mod complex;
mod draw;
mod error;
mod flag;
mod help;
mod state;
mod util;
mod variant;

use crate::{
	flag::Flag,
	state::State
};

static VERSION: &str = env!("CARGO_PKG_VERSION");

pub const FLAG_HELP:	[&str;2]	= [ "-h", "--help" ];
pub const FLAG_LIST:	[&str;2]	= [ "-l", "--list" ];
pub const FLAG_SIZE:	[&str;2]	= [ "-s", "--size"];
pub const FLAG_VERSION:	[&str;2]	= [ "-v", "--version" ];

fn main() {
	//	collect args
	let mut args = Arguments::from_env();

	//	handle help flag
	if args.contains(FLAG_HELP) {
		let target = args.subcommand().unwrap();
		if target.is_some() { help::flag_help(&target.unwrap()); }
		else { help::help_text(); }
		return;
	}

	//	handle list flag
	if args.contains(FLAG_LIST) {
		help::list_text();
		return;
	}

	//	handle version flag
	if args.contains(FLAG_VERSION) {
		println!("pride v{VERSION}");
		return;
	}

	let state = State::new(&mut args);

	let subcommand =
		if let Ok(Some(subcommand)) = args.subcommand() { Some(subcommand) }
		else if let Ok(default) = var("PRIDE_DEFAULT") {
			if default.is_empty() { None }
			else { Some(default) }
		} else { None };
	let variant = args.subcommand().unwrap();

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
						=>	complex::progress(&state),
					_
						=>	flag::pride()
				}
			},

		Some("progress")
			=>	complex::progress(&state),


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
						=>	complex::aroace_halves(&state),
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
			=>	complex::demiromantic(&state),

		Some("demisexual")
			=>	complex::demisexual(&state),

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


		Some("lesbian" | "wlw")
			=>	{
				match variant.as_deref() {
					Some("7" | "7-color" | "7-stripe")
						=>	variant::lesbian_7(),
					_
						=>	flag::lesbian()
				}
			}


		Some("multisexual" | "m-spec" | "mspec")
			=>	flag::multisexual(),

		Some("multigender")
			=>	flag::multigender(),


		Some("neutrois")
			=>	flag::neutrois(),

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


		_
			=>	{ error::unmatched_flag(subcommand.unwrap()); panic!() }
	};

	//	draw flag
	flag.draw(&state);

}

