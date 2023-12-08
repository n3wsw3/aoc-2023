use std::collections::HashMap;

use itertools::FoldWhile::Continue;
use itertools::FoldWhile::Done;
use itertools::Itertools;

advent_of_code::solution!(8);

fn parse_map(input: &str) -> HashMap<&str, (&str, &str)> {
	input
		.lines()
		.map(|line| {
			// parse "TFN = (SMC, LQT)" into from = TFN, to = (SMC, LQT)
			let (from, to) = line.split_once(" = ").unwrap();
			// parse "(SMC, LQT)" into (SMC, LQT)
			let to = to.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
			// parse "SMC, LQT" into ("SMC", "LQT")
			let to = to.split(", ").collect_tuple::<(_, _)>().unwrap();

			(from, to)
		})
		.collect::<HashMap<_, _>>()
}

fn steps(steps: &str, start: &str, map: &HashMap<&str, (&str, &str)>) -> u64 {
	steps
		.chars()
		.cycle()
		.fold_while((start, 0), |(node, acc), direction| {
			if node.ends_with('Z') {
				return Done((node, acc));
			}

			let to = map.get(node).unwrap();

			Continue((
				match direction {
					'L' => to.0,
					'R' => to.1,
					_ => unreachable!(),
				},
				acc + 1,
			))
		})
		.into_inner()
		.1
}

fn lcm(a: u64, b: u64) -> u64 {
	a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
	if b == 0 {
		return a;
	}
	gcd(b, a % b)
}

pub fn part_one(input: &str) -> Option<u64> {
	let (instructions, map) = input.split_once("\n\n").unwrap();
	let map = parse_map(map);

	Some(steps(instructions, "AAA", &map))
}

pub fn part_two(input: &str) -> Option<u64> {
	let (instructions, map) = input.split_once("\n\n").unwrap();
	let map = parse_map(map);

	map.keys()
		.filter(|key| key.ends_with('A'))
		.map(|start| steps(instructions, start, &map))
		.reduce(lcm)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(2));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file_part(
			"examples", DAY, 2,
		));
		assert_eq!(result, Some(6));
	}
}
