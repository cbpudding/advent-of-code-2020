use ::std::{fs::File, io::{BufRead, BufReader}};

fn main() {
	let input = File::open("data.txt").unwrap();
	let reader = BufReader::new(input);
	let mut list = Vec::new();
	for line in reader.lines() {
		list.push(line.unwrap().parse::<usize>().unwrap());
	}
	let (a, b) = find_values(list);
	println!("{} + {} = {}", a, b, a + b);
	println!("{} * {} = {}", a, b, a * b);
}

fn find_values(list: Vec<usize>) -> (usize, usize) {
	for i in 0..(list.len() - 1) {
		for j in (i + 1)..(list.len() - 1) {
			if list[i] + list[j] == 2020 {
				return (list[i], list[j])
			}
		}
	}
	(0, 0)
}
