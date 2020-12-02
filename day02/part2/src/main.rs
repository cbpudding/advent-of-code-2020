use ::std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let input = File::open("data.txt").unwrap();
	let reader = BufReader::new(input);
	let mut valid = 0;
	for line in reader.lines() {
		let mut first = 0;
		let mut password = String::new();
		let mut second = 0;
		let mut victim = '\0';
		let mut stage = 0;
		for c in line.unwrap().chars() {
			if match stage {
				0 => {
					if c.is_digit(10) {
						first = (first * 10) + c.to_digit(10).unwrap() as usize;
						false
					} else {
						true
					}
				}
				1 => {
					if c.is_digit(10) {
						second = (second * 10) + c.to_digit(10).unwrap() as usize;
						false
					} else {
						true
					}
				}
				2 => {
					if victim == '\0' {
						victim = c;
						false
					} else {
						true
					}
				}
				_ => {
					if c.is_alphabetic() {
						password.push(c);
						false
					} else {
						false
					}
				}
			} {
				stage += 1;
			}
		}
		let first_match = password.chars().nth(first) == Some(victim);
		let second_match = password.chars().nth(second) == Some(victim);
		if first_match != second_match {
			valid += 1;
		}
	}
	println!("{} passwords are valid.", valid);
}
