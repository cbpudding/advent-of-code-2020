use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader},
};

fn is_valid(key: &[usize], val: usize) -> bool {
	for i in 0..(key.len() - 1) {
		for j in i..key.len() {
			if key[i] + key[j] == val {
				return true;
			}
		}
	}
	false
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut data = Vec::new();
	for l in reader.lines() {
		let line = l.unwrap();
		data.push(line.parse::<usize>().unwrap());
	}
	let body = &data[25..];
	for i in 0..body.len() {
		if !is_valid(&data[i..(i + 25)], body[i]) {
			println!("Invalid: {}", body[i]);
		}
	}
}
