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
	for l in reader.lines() {
		let line = l.unwrap();
		let opcode: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
		instructions.push((opcode, false));
	}
	'test: for i in 0..instructions.len() {
		for j in 0..instructions.len() {
			instructions[j].1 = false;
		}
		let opcode = &mut instructions[i];
		match opcode.0[0].as_str() {
			"acc" => continue,
			"jmp" => opcode.0[0] = "nop".to_string(),
			"nop" => opcode.0[0] = "jmp".to_string(),
			_ => panic!("Invalid opcode!"),
		}
		let (acc, success) = run(&mut instructions);
		if success {
			println!("Success! Instruction modified at {}!", i);
			println!("Accumulator value: {}", acc);
			break 'test;
		} else {
			let opcode = &mut instructions[i];
			match opcode.0[0].as_str() {
				"jmp" => opcode.0[0] = "nop".to_string(),
				"nop" => opcode.0[0] = "jmp".to_string(),
				_ => panic!("Invalid opcode!"),
			}
		}
	}
}

fn run(instructions: &mut Vec<(Vec<String>, bool)>) -> (isize, bool) {
	let mut acc = 0;
	let mut pc = 0;
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
	(acc, (pc as usize) >= instructions.len())
}
