use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader}
};

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut count = 0;
	let mut yes = Vec::new();
	for l in reader.lines() {
		let line = l.unwrap();
		if line == "" {
			count += yes.len();
			yes = Vec::new();
		} else {
			for c in line.chars() {
				if !yes.contains(&c) {
					yes.push(c);
				}
			}
		}
	}
	count += yes.len();
	println!("{} groups answered \"yes\" to a question.", count);
}
