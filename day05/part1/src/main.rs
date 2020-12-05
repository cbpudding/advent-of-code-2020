use ::std::{cmp, env, fs::File, io::{BufRead, BufReader}};

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut max = 0;
	for l in reader.lines() {
		let mut id = 0;
		for c in l.unwrap().chars() {
			match c {
				'B' => id = (id << 1) | 1,
				'F' => id <<= 1,
				'L' => id <<= 1,
				'R' => id = (id << 1) | 1,
				_ => panic!("Unknown character: {}", c)
			}
		}
		max = cmp::max(max, id);
	}
	println!("The largest seat ID is {}", max);
}
