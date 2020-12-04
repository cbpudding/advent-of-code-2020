use ::std::{collections::HashMap, env, fs::File, io::Read};

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const REQUIRED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut input = File::open(&args[1]).unwrap();
	let mut buffer = String::new();
	input.read_to_string(&mut buffer).unwrap();
	let passports: Vec<String> = buffer
		.trim()
		.split("\n\n")
		.map(|x| x.replace("\n", " ").to_string())
		.collect();
	let mut count = 0;
	for p in passports {
		let mut fields = HashMap::new();
		let raw: Vec<&str> = p.split(" ").collect();
		let mut valid = true;
		for f in raw {
			let mut field = f.split(":");
			let key = field.next().unwrap().to_string();
			let value = field.next().unwrap().to_string();
			fields.insert(key, value);
		}
		let keys: Vec<&String> = fields.keys().collect();
		for r in &REQUIRED {
			if keys.contains(&&r.to_string()) {
				let value = fields.get(&r.to_string()).unwrap();
				valid &= match *r {
					"byr" => value
						.parse::<usize>()
						.map_or(false, |year| year >= 1920 && year <= 2002),
					"iyr" => value
						.parse::<usize>()
						.map_or(false, |year| year >= 2010 && year <= 2020),
					"eyr" => value
						.parse::<usize>()
						.map_or(false, |year| year >= 2020 && year <= 2030),
					"hgt" => {
						let unit = &value[(value.len() - 2)..value.len()];
						let measure = &value[..(value.len() - 2)];
						match unit {
							"cm" => measure
								.parse::<usize>()
								.map_or(false, |height| height >= 150 && height <= 193),
							"in" => measure
								.parse::<usize>()
								.map_or(false, |height| height >= 59 && height <= 76),
							_ => false,
						}
					}
					"hcl" => {
						if &value[0..1] == "#" {
							let color = &value[1..];
							let mut valid = true;
							valid &= color.len() == 6;
							if valid {
								for c in color.chars() {
									valid &= c.is_digit(16);
								}
							}
							valid
						} else {
							false
						}
					}
					"ecl" => EYE_COLORS.contains(&value.as_str()),
					"pid" => value
						.parse::<usize>()
						.map_or(false, |id| id >= 100_000_000 && id <= 999_999_999),
					_ => true,
				};
			} else {
				valid = false;
			}
		}
		if valid {
			count += 1;
		}
	}
	println!("There are {} valid passports!", count);
}
