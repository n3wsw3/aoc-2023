use itertools::Itertools;
use std::{cell::RefCell, str::FromStr};

advent_of_code::solution!(4);

struct Game {
	winning_numbers: Vec<u32>,
	next_cards: Vec<usize>,
	win_amount: RefCell<Option<u32>>,
}

impl Game {
	fn win_amount(&self) -> u32 {
		self.winning_numbers
			.iter()
			.fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
	}
	fn win_cards(&self, games: &Vec<Game>) -> u32 {
		let mut wins = self.win_amount.borrow_mut();
		if wins.is_none() {
			*wins = Some(
				self.next_cards
					.iter()
					.map(|id| games[*id - 1].win_cards(games))
					.sum::<u32>() + 1,
			);
		}
		wins.unwrap()
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
			winning_numbers: numbers,
			next_cards: (1 + id..=wins + id).collect(),
			win_amount: RefCell::new(None),
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

	Some(games.iter().map(|game| game.win_cards(&games)).sum())
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
