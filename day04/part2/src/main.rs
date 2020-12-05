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
				let old = valid;
				valid &= match *r {
					"byr" => value
						.parse::<usize>()
						.map_or(false, |year| (1920..2003).contains(&year)),
					"iyr" => value
						.parse::<usize>()
						.map_or(false, |year| (2010..2021).contains(&year)),
					"eyr" => value
						.parse::<usize>()
						.map_or(false, |year| (2020..2031).contains(&year)),
					"hgt" => {
						let unit = &value[(value.len() - 2)..value.len()];
						let measure = &value[..(value.len() - 2)];
						match unit {
							"cm" => measure
								.parse::<usize>()
								.map_or(false, |height| (150..194).contains(&height)),
							"in" => measure
								.parse::<usize>()
								.map_or(false, |height| (59..77).contains(&height)),
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
					"pid" => {
						value
							.parse::<usize>()
							.map_or(false, |id| id < 1_000_000_000)
							&& value.len() == 9
					}
					_ => true,
				};
				if valid != old {
					println!("DEBUG: {} {}", r, value);
				}
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
