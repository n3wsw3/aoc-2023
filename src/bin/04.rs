use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(4);

struct Game {
	id: usize,
	winning_numbers: Vec<u32>,
	next_cards: Vec<usize>,
}

impl Game {
	fn win_amount(&self) -> u32 {
		self.winning_numbers
			.iter()
			.fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
	}
	fn win_cards(&self) -> Vec<usize> {
		self.next_cards.clone()
	}
}

impl FromStr for Game {
	type Err = String;

	fn from_str(str: &str) -> Result<Game, String> {
		// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
		let (card, game) = str.split_once(": ").ok_or("Invalid card")?;
		let (winning, numbers) = game.split_once(" | ").ok_or("Invalid card")?;

		let id = card[5..].trim().parse::<usize>().unwrap();

		let winning = winning
			.split(' ')
			.filter(|s| !s.is_empty())
			.map(|s| s.parse::<u32>().unwrap())
			.sorted()
			.collect_vec();

		let numbers = numbers
			.split(' ')
			.filter(|s| !s.is_empty())
			.map(|s| s.parse::<u32>().unwrap())
			.filter(|n| winning.binary_search(n).is_ok())
			.collect::<Vec<_>>();

		let wins = numbers.len();

		Ok(Game {
			id,
			winning_numbers: numbers,
			next_cards: (1 + id..=wins + id).collect(),
		})
	}
}

pub fn part_one(input: &str) -> Option<u32> {
	Some(
		input
			.lines()
			.map(|line| line.parse::<Game>().unwrap())
			.map(|game| game.win_amount())
			.sum(),
	)
}

pub fn part_two(input: &str) -> Option<u32> {
	let games: Vec<Game> = input
		.lines()
		.map(|line| line.parse::<Game>().unwrap())
		.collect();

	let mut ids = games.iter().map(|game| game.id).collect_vec();

	let mut i = 0;
	while i < ids.len() {
		let id = ids[i];
		let game = &games[id - 1];

		ids.append(&mut game.win_cards());
		i += 1;
	}

	Some(ids.len() as u32)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(13));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(30));
	}
}
