use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader},
};

fn check(map: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
	map[y][x % map[y].len()]
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let x_mul: usize = args[2].parse().unwrap();
	let y_mul: usize = args[3].parse().unwrap();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut map = Vec::new();
	for line in reader.lines() {
		let mut inner = Vec::new();
		for c in line.unwrap().chars() {
			match c {
				'.' => inner.push(false),
				'#' => inner.push(true),
				_ => panic!("What is this thing?"),
			}
		}
		map.push(inner);
	}
	let mut count = 0;
	for i in 0..map.len() {
		let x = i * x_mul;
		let y = i * y_mul;
		if y <= map.len() {
			count += check(&map, x, y) as usize;
		}
	}
	println!("You hit {} trees.", count);
}
