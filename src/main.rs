use std::{
	io::{ stdout, IsTerminal },
	process::exit
};

use pico_args::Arguments;

mod color;
mod complex;
mod draw;
mod flag;
mod util;
mod variant;

use crate::flag::Flag;

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	//	collect args
	let mut args = Arguments::from_env();

	//	handle help flag
	if args.contains(["-h", "--help"]) {
		help_text();
		return;
	}

	//	handle list flag
	if args.contains(["-l", "--list"]) {
		list_text();
		return;
	}

	//	handle version flag
	if args.contains("--version") {
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
			=>	flag::lesbian(),

		Some("multisexual" | "m-spec" | "mspec")
			=>	flag::multisexual(),

		Some("multigender")
			=>	flag::multigender(),

		Some("nonbinary" | "nb")
			=>	flag::nonbinary(),

		Some("pansexual" | "pan")
			=>	flag::pansexual(),

		Some("polyamory" | "polyamorous" | "poly")
			=>	complex::polyamory(small),

		Some("polysexual")
			=>	flag::polysexual(),

		Some("transgender" | "trans")
			=>	flag::transgender(),

		_ => { help_text(); exit(1) }
	};

	//	draw flag
	flag.draw(!small);

}

fn help_text() {
	println!("pride v{VERSION}");
	println!("Valerie Wolfe <sleeplessval@gmail.com>");
	println!("Show pride flags in the terminal.\n");

	println!("usage: pride [flags] [name]\n");

	println!("args:");
	println!("   <name>         The pride flag to display\n");

	println!("flags:");
	println!("   -h, --help     Shows this help text");
	println!("   --version      Show version information");
	println!("   -l, --list     Prints a list of printable flags");
	println!("   -s, --small    Prints a small version without holding");

	println!("\nUse 'pride --list' to see a list of printable flags");
	println!("\n~ You're loved and you matter ♥");
}

fn list_text() {
	println!("pride v{}", env!("CARGO_PKG_VERSION"));
	println!("\nFlag list:");
	println!("   agender                agender pride flag");
	println!("   aro, aromantic         aromantic pride flag");
	println!("   ace, asexual           asexual pride flag");
	println!("   aroace                 aromantic/asexual pride flag");
	println!("   bigender               bigender pride flag");
	println!("   bi, bisexual           bisexual pride flag");
	println!("   demiromantic           demiromantic pride flag");
	println!("   demisexual             demisexual pride flag");
//	println!("   disability             disability pride flag");
	println!("   gay, mlm               gay men pride flag");
	println!("   genderfluid            genderfluid pride flag");
	println!("   gender-nonconforming   gender nonconforming pride flag");
	println!("   genderqueer            genderqueer pride flag");
	println!("   gendervoid             gendervoid pride flag");
//	println!("   intersex               intersex pride flag");
	println!("   lesbian                lesbian pride flag");
	println!("   multisexual            multisexual pride flag");
	println!("   multigender            multigender pride flag");
	println!("   nb, nonbinary          nonbinary pride flag");
	println!("   pan, pansexual         pansexual pride flag");
	println!("   polyamory              polyamorous pride flag");
	println!("   polysexual             polysexual pride flag");
	println!("   pride, rainbow         six-color rainbow flag");
	println!("   progress               progress arrow flag");
	println!("   trans, transgender     transgender pride flag");
}

