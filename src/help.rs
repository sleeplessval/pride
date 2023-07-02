
use crate::VERSION;


pub fn help_text() {
	println!("pride v{VERSION}
Valerie Wolfe <sleeplessval@gmail.com>
Display pride flags in the terminal.

usage: pride [flags] [name]

args:
   <name>         The pride flag to display

flags:
   -h, --help     Shows this help text
   --version      Show version information
   -l, --list     Prints a list of printable flags
   -s, --small    Prints a small version without holding

Use 'pride --list' to see a list of printable flags

~ You're loved and you matter ♥");
}

pub fn list_text() {
	println!("pride v{VERSION}

flag list:
   agender                agender pride flag
   aro, aromantic         aromantic pride flag
   ace, asexual           asexual pride flag
   bigender               bigender pride flag
   bi, bisexual           bisexual pride flag
   gay, mlm               gay men pride flag
   genderfluid            genderfluid pride flag
   gender-nonconforming   gender nonconforming pride flag
   genderqueer            genderqueer pride flag
   gendervoid             gendervoid pride flag
   lesbian                lesbian pride flag
   multigender            multigender pride flag
   nb, nonbinary          nonbinary pride flag
   pan, pansexual         pansexual pride flag
   pride, rainbow         six-color rainbow flag
   trans, transgender     transgender pride flag");
}

pub fn flag_help(flag: &str) {
	match flag {
		"pride" | "rainbow"
			=> {
			println!("The ubiquitous 1979 6-color rainbow pride flag, representing the larger queer community.

names:
   'pride', 'rainbow'

variants:
   8-color            Gilbert Baker's original 1978 flag with 8 stripes
   gilbert-baker
   sex-and-magic
   philadelphia       The 2017 Philadelphia Pride flag with black and brown stripes");
			},

		"transgender" | "trans"
			=> {
				println!("The transgender pride flag designed by Monica Helms in 1999.

names:
   'transgender', 'trans'
");
			}

		_
			=> help_text()
	}
}

