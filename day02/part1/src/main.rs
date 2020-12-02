use ::regex::Regex;
use ::std::{
	fs::File,
	io::{BufRead, BufReader},
};

struct Password {
	password: String,
	max: usize,
	min: usize,
	victim: char,
}

impl Password {
	fn is_valid(&self) -> bool {
		let count = count(&self.password, self.victim);
		self.min <= count && self.max >= count
	}
}

fn main() {
	let input = File::open("data.txt").unwrap();
	let reader = BufReader::new(input);
	let pattern = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
	let mut list = Vec::new();
	let mut valid = 0;
	for line in reader.lines() {
		let line = line.unwrap();
		let caps = pattern.captures(&line).unwrap();
		list.push(Password {
			password: caps.get(4).unwrap().as_str().to_string(),
			max: caps
				.get(2)
				.unwrap()
				.as_str()
				.to_string()
				.parse::<usize>()
				.unwrap(),
			min: caps
				.get(1)
				.unwrap()
				.as_str()
				.to_string()
				.parse::<usize>()
				.unwrap(),
			victim: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
		});
	}
	for pass in list {
		if pass.is_valid() {
			valid += 1;
		}
	}
	println!("{} passwords are valid.", valid);
}

fn count(password: &String, victim: char) -> usize {
	let mut count = 0;
	for c in password.chars() {
		if c == victim {
			count += 1;
		}
	}
	count
}
