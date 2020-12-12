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
	let (mut ship_x, mut ship_y) = (0, 0);
	let (mut way_x, mut way_y) = (10, 1);
	for l in reader.lines() {
		let line = l.unwrap();
		let value = line[1..].parse::<isize>().unwrap();
		match &line[0..1] {
			"N" => way_y += value,
			"S" => way_y -= value,
			"E" => way_x += value,
			"W" => way_x -= value,
			"L" => {
				let before = r;
				r = ((r - value) + 360) % 360;
				println!("{} -> {}({})", before, r, ((before - r) + 360) % 360);
				match ((before - r) + 360) % 360 {
					0 => {}
					90 => {
						let (x, y) = (way_x, way_y);
						way_x = y * -1;
						way_y = x;
					}
					180 => {
						way_x *= -1;
						way_y *= -1;
					}
					270 => {
						let (x, y) = (way_x, way_y);
						way_x = y;
						way_y = x * -1;
					}
					_ => panic!("Invalid rotation!"),
				}
			}
			"R" => {
				let before = r;
				r = (r + value) % 360;
				match ((r - before) + 360) % 360 {
					0 => {}
					90 => {
						let (x, y) = (way_x, way_y);
						way_x = y;
						way_y = x * -1;
					}
					180 => {
						way_x *= -1;
						way_y *= -1;
					}
					270 => {
						let (x, y) = (way_x, way_y);
						way_x = y * -1;
						way_y = x;
					}
					_ => panic!("Invalid rotation!"),
				}
			}
			"F" => {
				ship_x += way_x * value;
				ship_y += way_y * value;
			}
			_ => panic!("Invalid operation!"),
		}
	}
	println!("The manhattan distance is {}", ship_x.abs() + ship_y.abs());
}
