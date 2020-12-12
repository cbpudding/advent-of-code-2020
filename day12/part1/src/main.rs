use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut r = 90; // Facing East
	let (mut x, mut y) = (0, 0);
	for l in reader.lines() {
		let line = l.unwrap();
		let value = line[1..].parse::<isize>().unwrap();
		match &line[0..1] {
			"N" => y += value,
			"S" => y -= value,
			"E" => x += value,
			"W" => x -= value,
			"L" => r = ((r - value) + 360) % 360,
			"R" => r = (r + value) % 360,
			"F" => match r {
				0 => y += value,
				90 => x += value,
				180 => y -= value,
				270 => x -= value,
				_ => panic!("Invalid rotation!"),
			},
			_ => panic!("Invalid operation!"),
		}
	}
	println!("The manhattan distance is {}", x.abs() + y.abs());
}
