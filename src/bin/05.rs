use std::str::FromStr;

use itertools::Itertools;

advent_of_code::solution!(5);

struct Range {
	destination: u64,
	start: u64,
	length: u64,
}

impl Range {
	fn new(destination: u64, start: u64, length: u64) -> Self {
		Self {
			destination,
			start,
			length,
		}
	}
}

impl FromStr for Range {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (destination, start, length) = s
			.split(' ')
			.map(|num| num.parse::<u64>().unwrap())
			.collect_tuple::<(_, _, _)>()
			.unwrap();
		Ok(Range::new(destination, start, length))
	}
}

struct Map {
	ranges: Vec<Range>,
}

impl Map {
	fn new_with_range(ranges: Vec<Range>) -> Self {
		Self { ranges }
	}

	fn map(&self, nums: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
		nums.iter()
			.flat_map(|num| {
				let mut result: Vec<(u64, u64)> = Vec::new();
				let mut added: u64 = num.0;
				for range in &self.ranges {
					if added < range.start {
						let to = u64::min(range.start, num.1);
						result.push((added, to));
						added = to;
					}
					if added >= range.start && added < range.start + range.length {
						let to = u64::min(range.start + range.length, num.1);
						result.push((
							range.destination + added - range.start,
							range.destination + to - range.start,
						));
						added = to;
					}

					if added >= num.1 {
						break;
					}
				}
				if added < num.1 {
					result.push((added, num.1));
				}
				result
			})
			.collect_vec()
	}
}

impl FromStr for Map {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut ranges = s.lines();
		ranges.next();
		let x = ranges
			.map(|line| line.parse::<Range>().unwrap())
			.sorted_by_key(|range| range.start)
			.collect_vec();
		Ok(Map::new_with_range(x))
	}
}

struct Game {
	seeds: Vec<u64>,
	maps: Vec<Map>,
}

impl Game {
	fn lowest_seed_location(&self) -> u64 {
		let ranges = self.seeds.iter().map(|num| (*num, *num + 1)).collect_vec();

		self.maps
			.iter()
			.fold(ranges, |acc, map| map.map(acc))
			.iter()
			.min_by_key(|(start, _)| start)
			.unwrap()
			.0
	}

	fn lowest_seed_location_range(&self) -> u64 {
		let ranges = self
			.seeds
			.chunks(2)
			.map(|chunk| {
				let start = chunk[0];
				let length = chunk[1];
				(start, start + length)
			})
			.collect_vec();

		self.maps
			.iter()
			.fold(ranges, |acc, map| map.map(acc))
			.iter()
			.min_by_key(|(start, _)| start)
			.unwrap()
			.0
	}
}

impl FromStr for Game {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut input = s.split("\n\n");
		let seeds = input
			.next()
			.map(|line| {
				line.split(' ')
					.skip(1)
					.map(|num| num.parse::<u64>().unwrap())
			})
			.unwrap()
			.collect_vec();
		let maps = input
			.map(|block| block.parse::<Map>().unwrap())
			.collect_vec();

		Ok(Game { seeds, maps })
	}
}

pub fn part_one(input: &str) -> Option<u64> {
	input
		.parse::<Game>()
		.map(|game| game.lowest_seed_location())
		.ok()
}

pub fn part_two(input: &str) -> Option<u64> {
	input
		.parse::<Game>()
		.map(|game| game.lowest_seed_location_range())
		.ok()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(35));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(46));
	}
}
