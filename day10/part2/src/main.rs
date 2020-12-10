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
	let mut ranges = Vec::new();
	let mut streak = 0;
	for i in 0..adapters.len() {
		let delta = adapters[i] - joltage;
		match delta {
			1 => streak += 1,
			3 => {
				if streak != 0 {
					ranges.push(streak);
					streak = 0;
				}
			}
			_ => panic!("That wasn't supposed to happen! ~Some scottish demolitions man"),
		}
		joltage = adapters[i];
	}
	let combos = ranges
		.iter()
		.fold(1, |a, x| a * (2usize.pow((x - 1) as _) - ((x - 1) / 3)));
	println!("Combinations: {}", combos);
}
