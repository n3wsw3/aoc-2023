use std::str::FromStr;

use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

advent_of_code::solution!(5);

struct Range {
	destination: u32,
	start: u32,
	length: u32,
}

impl Range {
	fn new(destination: u32, start: u32, length: u32) -> Self {
		Self {
			destination,
			start,
			length,
		}
	}

	fn map(&self, num: u32) -> Option<u32> {
		if num >= self.start && num - self.start < self.length {
			Some(self.destination + (num - self.start))
		} else {
			None
		}
	}
}

impl FromStr for Range {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (destination, start, length) = s
			.split(' ')
			.map(|num| num.parse::<u32>().unwrap())
			.collect_tuple::<(_, _, _)>().unwrap();
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

	fn map(&self, num: u32) -> Option<u32> {
		for range in &self.ranges {
			if let Some(mapped) = range.map(num) {
				return Some(mapped);
			}
		}
		None
	}
}

impl FromStr for Map {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut ranges = s.lines();
		ranges.next();
		let x = ranges
			.map(|line| line.parse::<Range>().unwrap())
			.collect_vec();
		Ok(Map::new_with_range(x))
	}
}

struct Game {
	seeds: Vec<u32>,
	maps: Vec<Map>,
}

impl Game {
  fn lowest_seed_location(&self) -> u32 {
    self.seeds.iter().map(|seed| {
      self.maps.iter().fold(*seed, |acc, map| {
        map.map(acc).unwrap_or(acc)
      })
    }).min().unwrap()
  }

  fn lowest_seed_location_range(&self) -> u32 {
    self.seeds.chunks(2).par_bridge().map(|chunk| {
      let start = chunk[0];
      let length = chunk[1];
      (start..start+length).into_par_iter().map(|num| {
        self.maps.iter().fold(num, |acc, map| {
          map.map(acc).unwrap_or(acc)
        })
      }).min().unwrap()
    }).min().unwrap()
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
					.map(|num| num.parse::<u32>().unwrap())
			})
			.unwrap()
			.collect_vec();
		let maps = input
			.map(|block| block.parse::<Map>().unwrap())
			.collect_vec();

		Ok(Game { seeds, maps })
	}
}

pub fn part_one(input: &str) -> Option<u32> {
	input.parse::<Game>().map(|game| game.lowest_seed_location()).ok()
}

pub fn part_two(input: &str) -> Option<u32> {
  input.parse::<Game>().map(|game| game.lowest_seed_location_range()).ok()
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
