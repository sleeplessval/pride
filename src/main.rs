use std::process::exit;

use pico_args::Arguments;

mod color;
mod complex;
mod draw;
mod flag;
mod variant;

use crate::color::Colors;

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
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

	//	get small flag
	let small = args.contains(["-s", "--small"]);

	let subcommand = args.subcommand().unwrap();

	let colors: Colors = match subcommand.as_deref() {
		Some("pride" | "gay")
			=>	{
				let variant = args.subcommand().unwrap_or(None);
				match variant.as_deref() {
					Some("8-color" | "gilbert-baker" | "sex-and-magic")
						=>	variant::gilbert_baker(),
					Some("philadelphia")
						=>	variant::philadelphia(),
//					Some("progress")
//						=>	complex::progress(),
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

//		Some("aroace" | "aromantic-asexual")
//			=>	complex::aroace(),

		Some("bigender")
			=>	flag::bigender(),

		Some("bisexual" | "bi")
			=>	flag::bisexual(),

//		Some("demiromantic")
//			=>	complex::demiromantic(),

//		Some("demisexual")
//			=>	complex::demisexual(),

//		Some("disability")
//			=>	complex::disability();

		Some("genderfluid")
			=>	flag::genderfluid(),

		Some("genderqueer")
			=>	flag::genderqueer(),

		Some("gendervoid")
			=>	flag::gendervoid(),

		Some("intersex")
			=>	complex::intersex(),

		Some("lesbian")
			=>	flag::lesbian(),

		Some("multigender")
			=>	flag::multigender(),

		Some("nonbinary" | "nb")
			=>	flag::nonbinary(),

		Some("pansexual" | "pan")
			=>	flag::pansexual(),

//		Some("poly" | "polyamorous" | "polyamory")
//			=>	complex::polyamorous(),

		_ => { help_text(); exit(1) }
	};

	if small { draw::small(colors); }
	else { draw::full(colors); }

}

fn help_text() {
	println!("pride v{VERSION}");
	println!("Valerie Wolfe <sleeplessval@gmail.com>");
	println!("Show pride flags in the terminal.\n");

	println!("usage: pride [flags] <name>\n");

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
//	println!("   aroace                 aromantic/asexual pride flag");
	println!("   bigender               bigender pride flag");
	println!("   bi, bisexual           bisexual pride flag");
//	println!("   demiromantic           demiromantic pride flag");
//	println!("   demisexual             demisexual pride flag");
//	println!("   disability             disability pride flag");
	println!("   gay, pride             six-color rainbow flag");
	println!("   genderfluid            genderfluid pride flag");
	println!("   genderqueer            genderqueer pride flag");
	println!("   gendervoid             gendervoid pride flag");
//	println!("   intersex               intersex pride flag");
	println!("   lesbian                lesbian pride flag");
	println!("   multigender            multigender pride flag");
	println!("   nb, nonbinary          nonbinary pride flag");
	println!("   pan, pansexual         pansexual pride flag");
//	println!("   poly, polyamorous      polyamorous pride flag");
	println!("   trans, transgender     transgender pride flag");
}

