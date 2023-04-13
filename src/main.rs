use std::process::exit;

use pico_args::Arguments;

mod draw;
mod flag;

fn main() {
	let mut args = Arguments::from_env();

	let help = args.contains(["-h", "--help"]);
	if help {
		help_text();
		return;
	}

	let list = args.contains(["-l", "--list"]);
	if list {
		list_text();
		return;
	}
	
	let small = args.contains(["-s", "--small"]);

	let subcommand = args.subcommand().unwrap();

	match subcommand.as_deref() {
		Some("pride")		|
		Some("gay")			=>	flag::pride(small),

		Some("trans")		|
		Some("transgender")	=>	flag::transgender(small),


		Some("aro")			|
		Some("aromantic")	=>	flag::aromantic(small),

		Some("ace")			|
		Some("asexual")		=>	flag::asexual(small),

		Some("bigender")	=>	flag::bigender(small),

		Some("bi")			|
		Some("bisexual")	=>	flag::bisexual(small),

		Some("gendervoid")	=>	flag::gendervoid(small),

		Some("lesbian")		=>	flag::lesbian(small),

		Some("multigender")	=>	flag::multigender(small),

		Some("nb")			|
		Some("nonbinary")	=>	flag::nonbinary(small),

		Some("pan")			|
		Some("pansexual")	=>	flag::pansexual(small),


		Some("sex-and-magic")|
		Some("baker")		 |
		Some("gilbert")		=>	flag::gilbert(small),

		Some("philly")		|
		Some("philadelphia")=>	flag::philadelphia(small),

		_ => { help_text(); exit(1) }
	}
}

fn help_text() {
	println!("pride v{}", env!("CARGO_PKG_VERSION"));
	println!("Valerie Wolfe <sleeplessval@gmail.com>");
	println!("Show pride flags in the terminal.\n");

	println!("usage: pride [flags] <name>\n");

	println!("args:");
	println!("   <name>         The pride flag to display\n");

	println!("flags:");
	println!("   -h, --help     Shows this help text");
	println!("   -l, --list     Prints a list of printable flags");
	println!("   -s, --small    Prints a small version without holding");

	println!("\nUse 'pride --list' to see a list of printable flags");
	println!("\n~ You're loved and you matter ♥");
}

fn list_text() {
	println!("pride v{}", env!("CARGO_PKG_VERSION"));
	println!("\nFlag list:");
	println!("   aro, aromantic         aromantic pride flag");
	println!("   ace, asexual           asexual pride flag");
	println!("   bigender               bigender pride flag");
	println!("   bi, bisexual           bisexual pride flag");
	println!("   gay, pride             six-color rainbow flag");
	println!("   gendervoid             gendervoid pride flag");
	println!("   lesbian                lesbian pride flag");
	println!("   multigender            multigender pride flag");
	println!("   nb, nonbinary          nonbinary pride flag");
	println!("   pan, pansexual         pansexual pride flag");
//	println!("   progress               progress arrow flag");
	println!("   trans, transgender     transgender pride flag");
}

