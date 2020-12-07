use ::std::{
	collections::HashMap,
	env,
	fs::File,
	io::{BufRead, BufReader},
};

fn can_contain(
	hierarchy: &HashMap<String, Vec<(usize, String)>>,
	color: &String,
	checked: Option<&Vec<String>>,
) -> Vec<String> {
	let visited: Vec<String> = if let Some(c) = checked {
		c.clone()
	} else {
		Vec::new()
	};
	let mut containers = Vec::new();
	for c in visited {
		containers.push(c);
	}
	for k in hierarchy.keys() {
		if !containers.contains(&k) {
			for b in hierarchy.get(k).unwrap() {
				if b.1 == *color {
					containers.push(k.clone());
					for c in can_contain(&hierarchy, &k, Some(&containers)) {
						if !containers.contains(&c) {
							containers.push(c);
						}
					}
				}
			}
		}
	}
	containers
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
	let containers = can_contain(&hierarchy, &String::from("shiny gold"), None);
	println!(
		"The shiny gold bag can be contained in {} differently colored bags",
		containers.len()
	);
}
