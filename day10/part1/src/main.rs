use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut adapters = Vec::new();
	for l in reader.lines() {
		let line = l.unwrap();
		adapters.push(line.parse::<usize>().unwrap());
	}
	adapters.sort();
	adapters.push(adapters[adapters.len() - 1] + 3);
	let mut joltage = 0;
	let mut diff = (0, 0, 0);
	for i in 0..adapters.len() {
		let delta = adapters[i] - joltage;
		match delta {
			1 => diff.0 += 1,
			2 => diff.1 += 1,
			3 => diff.2 += 1,
			_ => panic!("Joltage out of range!"),
		}
		joltage = adapters[i];
	}
	println!("Differences: ({}, {}, {})", diff.0, diff.1, diff.2);
	println!("Product of 1 and 3 jolt differences: {}", diff.0 * diff.2);
}
