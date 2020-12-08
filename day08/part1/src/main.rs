use ::std::{
	env,
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	let args: Vec<String> = env::args().collect();
	let input = File::open(&args[1]).unwrap();
	let reader = BufReader::new(input);
	let mut instructions = Vec::new();
	let mut acc = 0;
	let mut pc = 0;
	for l in reader.lines() {
		let line = l.unwrap();
		let opcode: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
		instructions.push((opcode, false));
	}
	'main: while (pc as usize) < instructions.len() {
		let mut opcode = &mut instructions[pc as usize];
		if opcode.1 {
			break 'main;
		}
		match opcode.0[0].as_str() {
			"acc" => acc += opcode.0[1].parse::<isize>().unwrap(),
			"jmp" => pc += opcode.0[1].parse::<isize>().unwrap() - 1,
			"nop" => {}
			_ => panic!("Invalid opcode!"),
		}
		opcode.1 = true;
		pc += 1;
	}
	println!("Accumulator value: {}", acc);
}
