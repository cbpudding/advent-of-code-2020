use ::std::{
	cmp, env,
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
	let min_x = cmp::max(0, x as isize - 1) as usize;
	let min_y = cmp::max(0, y as isize - 1) as usize;
	let max_x = cmp::min(grid[0].len() - 1, x + 1);
	let max_y = cmp::min(grid.len() - 1, y + 1);
	let mut adjacent = 0;
	for a in min_y..(max_y + 1) {
		for b in min_x..(max_x + 1) {
			if (b, a) == (x, y) {
				if grid[a][b] == Space::Occupied {
					adjacent += 1;
				}
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
					if adjacent >= 4 {
						new[y][x] = Space::Seat;
					}
				}
			}
		}
	}
	new
}
