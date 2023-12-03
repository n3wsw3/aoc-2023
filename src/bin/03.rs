use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(3);

fn find_adjacent(
	i: usize,
	j: usize,
	row_length: usize,
	col_length: usize,
) -> impl Iterator<Item = (usize, usize)> + 'static {
	let i_range = i.saturating_sub(1)..=usize::min(i + 1, col_length - 1);
	let j_range = j.saturating_sub(1)..=usize::min(j + 1, row_length - 1);
	i_range.flat_map(move |k| j_range.clone().map(move |l| (k, l)))
}

fn solve<Collect>(input: &str, filter: fn(char) -> bool, collect: Collect) -> Option<u32>
where
	Collect: Fn((&(usize, usize), &Vec<u32>)) -> Vec<u32>,
{
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

			is_part = is_part.or(find_adjacent(i, j, map.len(), map[1].len())
				.map(|(i, j)| ((i, j), map[i][j]))
				.find(|(_, c)| !c.is_ascii_digit() && filter(*c)));

			// if next element is not a digit
			if j + 1 >= map[i].len() || !map[i][j + 1].is_numeric() {
				if let Some((pos, _)) = is_part {
					sum.entry(pos).or_default().push(num);
				}
				is_part = None;
				num = 0;
			}
		}
	}
	Some(sum.iter().flat_map(collect).sum::<u32>())
}

pub fn part_one(input: &str) -> Option<u32> {
	solve(input, |c| c != '.', |(_, nums)| nums.to_vec())
}

pub fn part_two(input: &str) -> Option<u32> {
	solve(
		input,
		|c| c == '*',
		|(_, nums)| {
			if nums.len() == 2 {
				vec![nums.iter().product()]
			} else {
				vec![]
			}
		},
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
