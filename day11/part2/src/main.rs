use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug)]
enum Space {
	Floor,
	Seat,
	Occupied,
}

impl PartialEq for Space {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Space::Floor, Space::Floor) => true,
			(Space::Seat, Space::Seat) => true,
			(Space::Occupied, Space::Occupied) => true,
			_ => false,
		}
	}
}

fn find_adjacent(grid: &Vec<Vec<Space>>, x: usize, y: usize) -> usize {
	let mut adjacent = 0;
	for a in -1..2 {
		for b in -1..2 {
			if (b, a) != (0, 0) {
				adjacent += trace_ray(&grid, x, y, (b, a)) as usize;
			}
		}
	}
	adjacent
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut grid = Vec::new();
	for l in reader.lines() {
		let line = l.unwrap();
		let mut row = Vec::new();
		for c in line.chars() {
			row.push(match c {
				'.' => Space::Floor,
				'L' => Space::Seat,
				'#' => Space::Occupied,
				_ => panic!("Unexpected character!"),
			});
		}
		grid.push(row);
	}
	let mut stable = false;
	while !stable {
		let new = tick(&grid);
		stable = grid == new;
		grid = new;
	}
	let mut count = 0;
	for r in grid {
		for c in r {
			if c == Space::Occupied {
				count += 1;
			}
		}
	}
	println!("There are {} occupied seats", count);
}

fn tick(grid: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
	let mut new = grid.clone();
	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			let adjacent = find_adjacent(&grid, x, y);
			match grid[y][x] {
				Space::Floor => (),
				Space::Seat => {
					if adjacent == 0 {
						new[y][x] = Space::Occupied;
					}
				}
				Space::Occupied => {
					if adjacent >= 5 {
						new[y][x] = Space::Seat;
					}
				}
			}
		}
	}
	new
}

fn trace_ray(grid: &Vec<Vec<Space>>, x: usize, y: usize, slope: (isize, isize)) -> bool {
	let a = y as isize + slope.1;
	let b = x as isize + slope.0;
	if a < 0 || a > grid.len() as isize - 1 {
		false
	} else if b < 0 || b > grid[0].len() as isize - 1 {
		false
	} else {
		let a = a as usize;
		let b = b as usize;
		match grid[a][b] {
			Space::Occupied => true,
			Space::Seat => false,
			_ => trace_ray(&grid, b, a, slope),
		}
	}
}
