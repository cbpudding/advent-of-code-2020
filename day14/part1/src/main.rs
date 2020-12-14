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
	let mut memory = HashMap::new();
	for l in reader.lines() {
		let line = l.unwrap();
		let opcode: Vec<&str> = line.split(" = ").collect();
		match &opcode[0][0..4] {
			"mask" => {
				stage1 = 0;
				stage2 = 0;
				for c in opcode[1].chars() {
					match c {
						'0' => {
							stage1 <<= 1;
							stage2 <<= 1;
						}
						'1' => {
							stage1 <<= 1;
							stage2 = (stage2 << 1) | 1;
						}
						'X' => {
							stage1 = (stage1 << 1) | 1;
							stage2 <<= 1;
						}
						_ => panic!("Unknown character!"),
					}
				}
			}
			"mem[" => {
				let loc = opcode[0][4..(opcode[0].len() - 1)]
					.parse::<usize>()
					.unwrap();
				let val = opcode[1].parse::<usize>().unwrap();
				memory.insert(loc, (val & stage1) | stage2);
			}
			_ => panic!("Invalid instruction!"),
		}
	}
	let sum = memory.values().fold(0, |a, x| a + x);
	println!("The sum of all values in memory is {}", sum);
}
