use ::std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let input = File::open("data.txt").unwrap();
	let reader = BufReader::new(input);
	let mut valid = 0;
	for line in reader.lines() {
		let mut password = String::new();
		let mut max = 0;
		let mut min = 0;
		let mut victim = '\0';
		let mut stage = 0;
		for c in line.unwrap().chars() {
			if match stage {
				0 => {
					if c.is_digit(10) {
						min = (min * 10) + c.to_digit(10).unwrap();
						false
					} else {
						true
					}
				}
				1 => {
					if c.is_digit(10) {
						max = (max * 10) + c.to_digit(10).unwrap();
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
		let count = count(password, victim);
		if min <= count && max >= count {
			valid += 1;
		}
	}
	println!("{} passwords are valid.", valid);
}

fn count(password: String, victim: char) -> u32 {
	let mut count = 0;
	for c in password.chars() {
		if c == victim {
			count += 1;
		}
	}
	count
}
