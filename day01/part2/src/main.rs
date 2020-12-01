use ::std::{fs::File, io::{BufRead, BufReader}};

fn main() {
	let input = File::open("data.txt").unwrap();
	let reader = BufReader::new(input);
	let mut list = Vec::new();
	for line in reader.lines() {
		list.push(line.unwrap().parse::<usize>().unwrap());
	}
	let (a, b, c) = find_values(list);
	println!("{} + {} + {} = {}", a, b, c, a + b + c);
	println!("{} * {} * {} = {}", a, b, c, a * b * c);
}

fn find_values(list: Vec<usize>) -> (usize, usize, usize) {
	for i in 0..(list.len() - 1) {
		for j in (i + 1)..(list.len() - 1) {
			for k in (j + 1)..(list.len() - 1) {
				if list[i] + list[j] + list[k] == 2020 {
					return (list[i], list[j], list[k])
				}
			}
		}
	}
	(0, 0, 0)
}
