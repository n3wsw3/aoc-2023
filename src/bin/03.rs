use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(3);

fn is_adjacent(map: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
	(i.saturating_sub(1)..=usize::min(i + 1, map.len() - 1))
		.flat_map(|k| {
			(j.saturating_sub(1)..=usize::min(j + 1, map[k].len() - 1)).map(move |l| map[k][l])
		})
		.any(|c| !c.is_ascii_digit() && c != '.')
}

pub fn part_one(input: &str) -> Option<u32> {
	let map = input
		.lines()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut num = 0;
	let mut is_part = false;
	let mut sum: Vec<u32> = Vec::new();
	for i in 0..map.len() {
		for j in 0..map[i].len() {
			let element = map[i][j];
			if !element.is_numeric() {
				continue;
			}

			num *= 10;
			num += element.to_digit(10)?;

			if !is_part {
				is_part |= is_adjacent(&map, i, j);
			}

			// if next element is not a digit
			if j + 1 >= map[i].len() || !map[i][j + 1].is_numeric() {
				if is_part {
					sum.push(num);
				}
				is_part = false;
				num = 0;
			}
		}
	}
	Some(sum.iter().sum())
}

fn find_adjacent_gear(map: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(usize, usize, char)> {
	(i.saturating_sub(1)..=usize::min(i + 1, map.len() - 1))
		.flat_map(|k| {
			(j.saturating_sub(1)..=usize::min(j + 1, map[k].len() - 1))
				.map(move |l| (k, l, map[k][l]))
		})
		.find(|(_, _, c)| !c.is_ascii_digit() && *c == '*')
}

pub fn part_two(input: &str) -> Option<u32> {
	let map = input
		.lines()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut num = 0;
	let mut is_part = None;
	let mut sum: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
	for i in 0..map.len() {
		for j in 0..map[i].len() {
			let element = map[i][j];
			if !element.is_numeric() {
				continue;
			}

			num *= 10;
			num += element.to_digit(10)?;

			if is_part.is_none() {
				is_part = find_adjacent_gear(&map, i, j);
			}

			// if next element is not a digit
			if j + 1 >= map[i].len() || !map[i][j + 1].is_numeric() {
				if is_part.is_some() {
					let id = (is_part.unwrap().0, is_part.unwrap().1);
					sum.entry(id).or_default().push(num);
				}
				is_part = None;
				num = 0;
			}
		}
	}
	Some(
		sum.iter()
			.filter(|(_, vec)| vec.len() == 2)
			.map(|(_, vec)| vec.iter().product::<u32>())
			.sum(),
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(4361));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(467835));
	}
}
