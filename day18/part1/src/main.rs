use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader},
	str::Chars
};

fn calc(iter: &mut Chars) -> isize {
	let mut a = 0;
	let mut c = iter.next();
	let mut num = false;
	let mut op = '+';
	let mut temp = 0;
	while c != None {
		let ch = c.unwrap();
		if ch != ')' {
			match ch {
				'0' => temp *= 10,
				'1' => temp = (temp * 10) + 1,
				'2' => temp = (temp * 10) + 2,
				'3' => temp = (temp * 10) + 3,
				'4' => temp = (temp * 10) + 4,
				'5' => temp = (temp * 10) + 5,
				'6' => temp = (temp * 10) + 6,
				'7' => temp = (temp * 10) + 7,
				'8' => temp = (temp * 10) + 8,
				'9' => temp = (temp * 10) + 9,
				' ' => {
					num = !num;
					if num {
						match op {
							'+' => a += temp,
							'-' => a -= temp,
							'*' => a *= temp,
							'/' => a /= temp,
							_ => panic!("Invalid operation!")
						}
						temp = 0;
					}
				},
				'(' => temp = calc(iter),
				'+' => op = '+',
				'-' => op = '-',
				'*' => op = '*',
				'/' => op = '/',
				_ => panic!("Unknown character!")
			}
			c = iter.next();
		} else {
			c = None;
		}
		if c == None {
			num = !num;
			if num {
				match op {
					'+' => a += temp,
					'-' => a -= temp,
					'*' => a *= temp,
					'/' => a /= temp,
					_ => panic!("Invalid operation!")
				}
			}
		}
	}
	a
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut acc = 0;
	for l in reader.lines() {
		let line = l.unwrap();
		let ans = calc(&mut line.chars());
		acc += ans;
	}
	println!("The sum of all the problems is {}", acc);
}
