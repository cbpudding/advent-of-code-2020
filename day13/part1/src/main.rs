use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader},
};

fn find_earliest(start: usize, lines: &Vec<usize>) -> (usize, usize) {
	let mut current = start;
	loop {
		for l in lines {
			if current % l == 0 {
				return (*l, current - start);
			}
		}
		current += 1;
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut lines = reader.lines();
	let earliest = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
	let bus_lines: Vec<usize> = lines
		.next()
		.unwrap()
		.unwrap()
		.split(",")
		.filter(|x| x != &"x")
		.map(|x| x.parse::<usize>().unwrap())
		.collect();
	let (id, wait) = find_earliest(earliest, &bus_lines);
	println!("You must wait {} minutes for bus {}.", wait, id);
	println!("The answer is {}", id * wait);
}
