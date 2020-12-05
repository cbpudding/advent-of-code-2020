use ::std::{env, fs::File, io::{BufRead, BufReader}};

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut seats = Vec::new();
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
		seats.push(id);
	}
	seats.sort();
	for i in 1..(seats.len() - 1) {
		if seats[i] - seats[i - 1] == 2 {
			println!("Middle seat discovered: {}", seats[i] - 1);
		}
	}
}
