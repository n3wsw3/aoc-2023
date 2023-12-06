use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
	let (time, distance) = input.lines().collect_tuple::<(_, _)>().unwrap();
	let start_digit = time
		.find(char::is_numeric)
		.unwrap()
		.min(distance.find(char::is_numeric).unwrap());
	let time = time[start_digit..]
		.split_ascii_whitespace()
		.map(|num| num.parse::<u32>().unwrap())
		.collect_vec();
	let distance = distance[start_digit..]
		.split_ascii_whitespace()
		.map(|num| num.parse::<u32>().unwrap())
		.collect_vec();

	Some(
		time.iter()
			.zip(distance.iter())
			.map(|(t, d)| {
				(0..=*t)
					.map(move |hold| (t - hold) * hold)
					.filter(|val| val > d)
					.count() as u32
			})
			.product::<u32>(),
	)
}

pub fn part_two(input: &str) -> Option<u64> {
	let (time, distance) = input.lines().collect_tuple::<(_, _)>().unwrap();
	let start_digit = time
		.find(char::is_numeric)
		.unwrap()
		.min(distance.find(char::is_numeric).unwrap());
	let time = time[start_digit..]
		.chars()
		.filter(|c| !c.is_whitespace())
		.collect::<String>()
		.parse::<f64>()
		.unwrap();
	let distance = distance[start_digit..]
		.chars()
		.filter(|c| !c.is_whitespace())
		.collect::<String>()
		.parse::<f64>()
		.unwrap();

	let s = f64::sqrt(time * time - 4. * 1. * distance);

	Some(((-time - s) / -2.).ceil() as u64 - ((-time + s) / -2.).ceil() as u64)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(288));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(71503));
	}
}
