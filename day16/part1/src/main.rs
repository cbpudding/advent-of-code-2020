use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader}
};

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut limits: Vec<usize> = Vec::new();
	let mut tickets = Vec::new();
	let mut stage = 0;
	for l in reader.lines() {
		let line = l.unwrap();
		match stage {
			0 => {
				if line != "" {
					let base: Vec<&str> = line.split(": ").collect();
					let ranges: Vec<&str> = base[1].split(" or ").collect();
					let first: Vec<usize> = ranges[0]
						.split("-")
						.map(|x| x.parse::<usize>().unwrap())
						.collect();
					let second: Vec<usize> = ranges[1]
						.split("-")
						.map(|x| x.parse::<usize>().unwrap())
						.collect();
					for a in first[0]..(first[1] + 1) {
						if !limits.contains(&a) {
							limits.push(a);
						}
					}
					for b in second[0]..(second[1] + 1) {
						if !limits.contains(&b) {
							limits.push(b);
						}
					}
				} else {
					stage += 1;
				}
			}
			1 => stage += 1,
			2 => stage += 1,
			3 => stage += 1,
			4 => stage += 1,
			5 => {
				if line != "" {
					let ticket: Vec<usize> = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
					tickets.push(ticket);
				} else {
					stage += 1;
				}
			}
			_ => panic!("Invalid stage"),
		}
	}
	let mut invalid = 0;
	for t in tickets {
		for v in t {
			if !limits.contains(&v) {
				invalid += v;
				break
			}
		}
	}
	println!("The total of invalid numbers is {}", invalid);
}
