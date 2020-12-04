use ::std::{env, fs::File, io::Read};

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut input = File::open(&args[1]).unwrap();
	let mut buffer = String::new();
	input.read_to_string(&mut buffer).unwrap();
	let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	let passports: Vec<String> = buffer
		.trim()
		.split("\n\n")
		.map(|x| x.replace("\n", " ").to_string())
		.collect();
	let mut count = 0;
	for p in passports {
		let mut exists = Vec::new();
		let fields: Vec<&str> = p.split(" ").collect();
		let mut valid = true;
		for f in fields {
			let field = f.split(":").next().unwrap();
			exists.push(field);
		}
		for r in &required {
			if !exists.contains(&r) {
				valid = false;
			}
		}
		if valid {
			count += 1;
		}
	}
	println!("There are {} valid passports!", count);
}
