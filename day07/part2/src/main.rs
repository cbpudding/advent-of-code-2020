use ::std::{
	collections::HashMap,
	env,
	fs::File,
	io::{BufRead, BufReader},
};

fn contains(hierarchy: &HashMap<String, Vec<(usize, String)>>, color: &String) -> usize {
	let mut count = 1;
	let bag = hierarchy.get(color).unwrap();
	for c in bag {
		println!("{} -> {}", color, c.1);
		count += c.0 * contains(hierarchy, &c.1);
	}
	count
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut hierarchy: HashMap<String, Vec<(usize, String)>> = HashMap::new();
	for l in reader.lines() {
		let line = l.unwrap();
		let level0: Vec<&str> = line.split(" contain ").collect();
		let key = String::from(level0[0].split(" ").collect::<Vec<&str>>()[0..2].join(" "));
		let level1: Vec<&str> = level0[1][..(level0[1].len() - 1)].split(", ").collect();
		let mut list = Vec::new();
		if level1[0] != "no other bags" {
			for b in level1 {
				let level2: Vec<&str> = b.split(" ").collect();
				let quantity = level2[0].parse::<usize>().unwrap();
				let color = String::from(level2[1..3].join(" "));
				list.push((quantity, color));
			}
		}
		hierarchy.insert(key, list);
	}
	let containers = contains(&hierarchy, &String::from("shiny gold"));
	println!("The shiny gold bag can contain {} bags", containers - 1);
}
