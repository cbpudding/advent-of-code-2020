use ::std::{collections::HashMap, env, fs::File, io::Read};

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
		for r in &required {
			if keys.contains(&&r.to_string()) {
				let value = fields.get(&r.to_string()).unwrap();
				valid &= match *r {
					"byr" => {
						if let Ok(year) = value.parse::<usize>() {
							year >= 1920 && year <= 2002
						} else {
							false
						}
					}
					"iyr" => {
						if let Ok(year) = value.parse::<usize>() {
							year >= 2010 && year <= 2020
						} else {
							false
						}
					}
					"eyr" => {
						if let Ok(year) = value.parse::<usize>() {
							year >= 2020 && year <= 2030
						} else {
							false
						}
					}
					"hgt" => {
						let unit = &value[(value.len() - 2)..value.len()];
						let measure = &value[..(value.len() - 2)];
						match unit {
							"cm" => {
								if let Ok(height) = measure.parse::<usize>() {
									height >= 150 && height <= 193
								} else {
									false
								}
							}
							"in" => {
								if let Ok(height) = measure.parse::<usize>() {
									height >= 59 && height <= 76
								} else {
									false
								}
							}
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
					"ecl" => {
						["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str())
					}
					"pid" => {
						let mut valid = true;
						valid &= value.len() == 9;
						for c in value.chars() {
							valid &= c.is_digit(10);
						}
						valid
					}
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
