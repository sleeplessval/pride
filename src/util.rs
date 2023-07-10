
///	gets the substring of displayed characters of an ANSI formatted string
pub fn ansi_substr(source: &str, start: usize, end: usize) -> String {
	//	init output string
	let mut output = String::new();

	//	trackers
	let mut escaped = false;
	let mut index = 0;
	for character in source.chars() {
		//	escape character delimits start and end of ansi sequences
	    if character == '\u{1B}' {
			escaped = true;
			output.push(character);
		}
		//	push ALL escaped characters
		if escaped {
			output.push(character);
			//	and unset esc on m
			if character == 'm' { escaped = false; }
		}
		//	non-escaped characters must obey bounds
		else {
			if index < start {
				index += 1;
				continue;
			}

			output.push(character);
			index += 1;

			if index > end { break; }
		}
    }
	output
}

///	gets the number of displayed characters in an ANSI formatted string
pub fn ansi_len(source: &str) -> usize {
	let mut output = 0;
	let mut escaped = false;

	for character in source.chars() {
		if character == '\u{1B}' { escaped = true; }

		if !escaped { output += 1; }
		else if character == 'm' { escaped = false; }
	}

	output
}

