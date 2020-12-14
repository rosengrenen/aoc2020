use crate::lib::Solver;
use std::collections::HashMap;

pub struct Day14Solver;

impl Solver for Day14Solver {
	fn solve_part_one(&self, input: &str) -> i64 {
		let instructions = instructions(input);
		let mut mask = SetMaskArgs::default();
		let mut mem: HashMap<i64, i64> = HashMap::new();
		for instruction in instructions.iter() {
			match instruction {
				&Instruction::SetMask(mask_args) => mask = mask_args,
				&Instruction::Write(WriteArgs { address, value }) => {
					// First part clears bits that are set by mask and second part
					// sets mask bits
					let value = (value & !mask.mask_bits) | mask.override_bits;
					mem.insert(address, value);
				}
			}
		}
		mem.iter().fold(0, |p, c| p + c.1)
	}

	fn solve_part_two(&self, input: &str) -> i64 {
		let instructions = instructions(input);
		let mut mask = SetMaskArgs::default();
		let mut mem: HashMap<i64, i64> = HashMap::new();
		for instruction in instructions.iter() {
			match instruction {
				&Instruction::SetMask(mask_args) => mask = mask_args,
				&Instruction::Write(WriteArgs { address, value }) => {
					// The first part only save only masked bits, since the others are
					// and need to be initialised to 0's
					// The second part overwrites 0's with 1's if bitmask is 1
					let address =
						(address & mask.mask_bits) | (mask.mask_bits & mask.override_bits);

					// Indices of floating bits
					let mut floating_bit_indices = Vec::new();
					for i in 0..36 {
						// Inverting mask_bits gets floating bits
						if (!mask.mask_bits) & 1 << i != 0 {
							floating_bit_indices.push(i);
						}
					}

					let num_floating_bits = floating_bit_indices.len();
					// All possible ways to set the floating bits is 2^n where n is number
					// of floating bits
					for n in 0..2i64.pow(num_floating_bits as u32) {
						// Make copy of address for each iteration
						let mut address = address;
						for i in 0..num_floating_bits {
							// Bit is one, since all floating bits in address are intialised
							// to 0's we only need to upgrade from 0's to 1's
							if n & 1 << i != 0 {
								address |= 1 << floating_bit_indices[i];
							}
						}

						mem.insert(address, value);
					}
				}
			}
		}
		mem.iter().fold(0, |p, c| p + c.1)
	}
}

#[derive(Clone, Copy, Default)]
struct SetMaskArgs {
	mask_bits: i64,
	override_bits: i64,
}

struct WriteArgs {
	address: i64,
	value: i64,
}

enum Instruction {
	SetMask(SetMaskArgs),
	Write(WriteArgs),
}

fn instructions(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|line| {
			if let Some(mask) = line.strip_prefix("mask = ") {
				// 1=masked, 0=not masked
				let mut mask_bits = 0;
				for &c in mask.as_bytes() {
					mask_bits <<= 1;
					match c {
						b'0' | b'1' => mask_bits |= 1,
						_ => (),
					}
				}
				// What the masked bit should be overwritten to, bits outside mask
				// are just ignored, since they are always used in conjuction with
				// mask
				let mut override_bits = 0;
				for &c in mask.as_bytes() {
					override_bits <<= 1;
					match c {
						b'1' => override_bits |= 1,
						_ => (),
					}
				}
				Instruction::SetMask(SetMaskArgs {
					mask_bits,
					override_bits,
				})
			} else {
				let mut split = line.split("] = ");
				let address = split
					.next()
					.unwrap()
					.strip_prefix("mem[")
					.unwrap()
					.parse()
					.unwrap();
				let value = split.next().unwrap().parse().unwrap();
				Instruction::Write(WriteArgs { address, value })
			}
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lib::fetch_input;
	use test::Bencher;

	#[test]
	fn part_one_test_cases() {
		let input = include_str!("input.test1.txt");
		let solver = Day14Solver {};
		assert_eq!(solver.solve_part_one(input), 165);
	}

	#[test]
	fn part_two_test_cases() {
		let input = include_str!("input.test2.txt");
		let solver = Day14Solver {};
		assert_eq!(solver.solve_part_two(input), 208);
	}

	#[bench]
	fn bench_parse_instructions(bencher: &mut Bencher) {
		let input = fetch_input(14);
		bencher.iter(|| instructions(&input));
	}

	#[bench]
	fn bench_part_one(bencher: &mut Bencher) {
		let input = fetch_input(14);
		let solver = Day14Solver {};
		bencher.iter(|| solver.solve_part_one(&input));
	}

	#[bench]
	fn bench_part_two(bencher: &mut Bencher) {
		let input = fetch_input(14);
		let solver = Day14Solver {};
		bencher.iter(|| solver.solve_part_two(&input));
	}
}
