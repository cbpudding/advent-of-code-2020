use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader},
};

fn egcd(a: isize, b: isize) -> (isize, isize, isize) {
	if a == 0 {
		(b, 0, 1)
	} else {
		let (g, x, y) = egcd(b % a, a);
		(g, y - (b / a) * x, x)
	}
}

fn mod_inv(x: isize, n: isize) -> Option<isize> {
	let (g, x, _) = egcd(x, n);
	if g == 1 {
		Some((x % n + n) % n)
	} else {
		None
	}
}

fn chinese_remainder(residues: &Vec<isize>, modulii: &Vec<isize>) -> Option<isize> {
	let prod = modulii.iter().product::<isize>();
	let mut sum = 0;
	for (&residue, &modulus) in residues.iter().zip(modulii) {
		let p = prod / modulus;
		sum += residue * mod_inv(p, modulus)? * p;
	}
	Some(sum % prod)
}

fn solve_timestamp(lines: &Vec<Option<isize>>) -> Option<isize> {
	let mut ids = Vec::new();
	let mut residues = Vec::new();
	for i in 0..lines.len() {
		if let Some(v) = lines[i] {
			ids.push(v);
			residues.push(v - i as isize);
		}
	}
	chinese_remainder(&residues, &ids)
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut lines = reader.lines();
	lines.next(); // We're just reading this to get it out of the way
	let bus_lines: Vec<Option<isize>> = lines
		.next()
		.unwrap()
		.unwrap()
		.split(",")
		.map(|x| {
			if x != "x" {
				Some(x.parse::<isize>().unwrap())
			} else {
				None
			}
		})
		.collect();
	if let Some(ans) = solve_timestamp(&bus_lines) {
		println!("The answer is {}", ans);
	} else {
		println!("Unable to find an answer");
	}
}
