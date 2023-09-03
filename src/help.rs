
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
   aromantic              aromantic pride flag
   aroace                 aromantic-asexual pride flag
   asexual                asexual pride flag
   bigender               bigender pride flag
   bisexual               bisexual pride flag
   demiromantic           demiromantic pride flag
   demisexual             demisexual pride flag
   gay                    gay men pride flag
   genderfluid            genderfluid pride flag
   gender-nonconforming   gender nonconforming pride flag
   genderqueer            genderqueer pride flag
   gendervoid             gendervoid pride flag
   lesbian                lesbian pride flag
   multigender            multigender pride flag
   nonbinary              nonbinary pride flag
   pansexual              pansexual pride flag
   rainbow                six-color rainbow flag
   progress               progress arrow rainbow flag
   transgender            transgender pride flag");
}

pub fn flag_help(flag: &str) {
	match flag {
		"pride" | "rainbow" | "-"
			=> {
			println!("The ubiquitous 1979 6-color rainbow pride flag, representing the larger queer community.

names:
   'pride', 'rainbow', '-'

variants:
   8-color            Gilbert Baker's original 1978 flag with 8 stripes
   gilbert-baker
   sex-and-magic
   philadelphia       The 2017 Philadelphia Pride flag with black and brown stripes
   progress           The 2018 Progess rainbow pride flag designed by Daniel Quasar");
			},
		"progress"
			=> { println!("Daniel Quasar's 2018 Progress rainbow pride flag.\n\nnames:\n   'progress'"); }

		"transgender" | "trans"
			=> {
				println!("The transgender pride flag designed by Monica Helms in 1999.

names:
   'transgender', 'trans'");
			},

		//	alphabetical below this point

		"aromantic" | "aro"
			=> { println!("The aromantic pride flag.\n\nnames:\n   'aromantic', 'aro'"); }
		"asexual" | "ace"
			=> { println!("The asexual pride flag.\n\nnames:\n   'asexual', 'ace'"); }
		"aroace" | "aromantic-asexual"
			=> {
				println!("The aromantic-asexual pride flag designed by aroaesflags on tumblr.

names:
   'aroace', 'aromantic-asexual'

variants:
   halves            The side-by-side aromantic and asexual aroace flag
   side-by-side
   sbs

notes:
   Side-by-side flag currently only displays in terminals 20 lines or taller.");
			},

		"bisexual" | "bi"
			=> {
				println!("The bisexual flag designed by Michael Page in 1998.

names:
   'bisexual', 'bi'");
			},


		"gay" | "mlm"
			=> {
				println!("The 7-stripe gay men pride flag designed by @gayflagblog on tumblr in 2019.

names:
   'gay', 'mlm'");
			},

		"gender-nonconforming" | "gendernonconforming" | "gnc"
			=> { println!("The gender-nonconforming pride flag.\n\nnames:\n   'gender-nonconforming', 'gendernonconforming', 'gnc'"); },


		"lesbian"
			=> {
				println!("The 5-stripe lesbian flag designed by Emily Gwen in 2018.

names:
   'lesbian'

variants:
   7-color            7-stripe flag, also designed in 2018 by Emily Gwen");
			},


		"nonbinary" | "nb"
			=> {
				println!("The nonbinary pride flag designed by Kyle Rowan in 2014.

names:
   'nonbinary', 'nb'");
			},


		"pansexual" | "pan"
			=> {
				println!("The pansexual pride flag designed by Jasper V around 2010

names:
   'pansexual', 'pan'");
			},


		_
			=> { help_text(); }
	}
}

