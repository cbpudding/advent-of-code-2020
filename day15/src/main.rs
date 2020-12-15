use ::std::{
	collections::HashMap,
	env,
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut lines = reader.lines();
	let data: Vec<usize> = lines
		.next()
		.unwrap()
		.unwrap()
		.split(",")
		.map(|x| x.parse::<usize>().unwrap())
		.collect();
	let mut nums = HashMap::new();
	let mut last = 0;
	let goal = args[2].parse::<usize>().unwrap();
	for i in 0..data.len() {
		nums.insert(data[i], (i + 1, usize::MAX));
		last = data[i];
	}
	for i in (data.len() + 1)..(goal + 1) {
		let (l, p) = nums.get(&last).unwrap().clone();
		let current = if p == usize::MAX { 0 } else { l - p };
		let prev = match nums.get(&current) {
			Some((v, _)) => *v,
			None => usize::MAX,
		};
		nums.insert(current, (i, prev));
		last = current;
	}
	println!("The {}th number spoken is {}", goal, last);
}
