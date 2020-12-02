use ::regex::Regex;
use ::std::{
	fs::File,
	io::{BufRead, BufReader},
};

struct Password {
	password: String,
	first: usize,
	second: usize,
	victim: char,
}

impl Password {
	fn is_valid(&self) -> bool {
		let first = self.password.chars().nth(self.first - 1) == Some(self.victim);
		let second = self.password.chars().nth(self.second - 1) == Some(self.victim);
		first != second
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
			first: caps
				.get(2)
				.unwrap()
				.as_str()
				.to_string()
				.parse::<usize>()
				.unwrap(),
			second: caps
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
