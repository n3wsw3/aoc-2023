advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
	const INPUTS: [u32; 3] = [12u32, 13u32, 14u32];
	input
		.lines()
		.map(|line| {
			let (game_id, cubes) = line.split_once(": ")?;

			let games = cubes
				.split("; ")
				.map(|game| {
					game.split(", ")
						.map(|color| {
							let (num, cube_color) = color.split_once(' ').unwrap();
							(
								num.parse::<u32>().unwrap(),
								match cube_color.chars().next() {
									Some('r') => 0,
									Some('g') => 1,
									Some('b') => 2,
									_ => panic!("Invalid color"),
								},
							)
						})
						.all(|(num, color_id)| INPUTS[color_id] >= num)
				})
				.all(|game| game);
			let id = game_id.split_once(' ').unwrap().1.parse::<u32>().ok();
			match games {
				true => id,
				false => None,
			}
		})
		.filter(Option::is_some)
		.sum()
}

pub fn part_two(input: &str) -> Option<u32> {
	input
		.lines()
		.map(|line| {
			let (_, game) = line.split_once(": ")?;

			Some(
				game.split("; ")
					.map(|set| {
						set.split(", ")
							.map(|color| {
								let (num, cube_color) = color.split_once(' ').unwrap();
								(
									num.parse::<u32>().unwrap(),
									match cube_color.chars().next() {
										Some('r') => 0,
										Some('g') => 1,
										Some('b') => 2,
										_ => panic!("Invalid color"),
									},
								)
							})
							.fold([0, 0, 0], |mut acc, (num, color_id)| {
								if acc[color_id] < num {
									acc[color_id] = num;
								}
								acc
							})
					})
					.fold([0, 0, 0], |mut acc, set| {
						for i in 0..3 {
							if acc[i] < set[i] {
								acc[i] = set[i];
							}
						}
						acc
					})
					.iter()
					.product::<u32>(),
			)
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(8));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(2286));
	}
}
