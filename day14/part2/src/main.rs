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
	let mut stage1 = 0;
	let mut stage2 = 0;
	let mut floating = Vec::new();
	let mut memory = HashMap::new();
	for l in reader.lines() {
		let line = l.unwrap();
		let opcode: Vec<&str> = line.split(" = ").collect();
		match &opcode[0][0..4] {
			"mask" => {
				stage1 = 0;
				stage2 = 0;
				floating.clear();
				let mut i = 0;
				for c in opcode[1].chars() {
					match c {
						'0' => {
							stage1 = (stage1 << 1) | 1;
							stage2 <<= 1;
						}
						'1' => {
							stage1 <<= 1;
							stage2 = (stage2 << 1) | 1;
						}
						'X' => {
							stage1 <<= 1;
							stage2 <<= 1;
							floating.push(i);
						}
						_ => panic!("Unknown character!"),
					}
					i += 1;
				}
			}
			"mem[" => {
				let loc = opcode[0][4..(opcode[0].len() - 1)]
					.parse::<usize>()
					.unwrap();
				let val = opcode[1].parse::<usize>().unwrap();
				let base = (loc & stage1) | stage2;
				for v in 0..(2usize.pow(floating.len() as u32)) {
					let mut stage3 = 0;
					for i in 0..floating.len() {
						let f = floating[i];
						if v & (1 << i) > 0 {
							stage3 |= 1 << (35 - f);
						}
					}
					memory.insert(base | stage3, val);
				}
			}
			_ => panic!("Invalid instruction!"),
		}
	}
	let sum = memory.values().fold(0, |a, x| a + x);
	println!("The sum of all values in memory is {}", sum);
}
