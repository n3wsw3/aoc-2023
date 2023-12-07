use std::{
	cmp::Ordering,
	collections::{HashMap, HashSet},
	hash::Hash,
};

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, Clone)]
struct Hand {
	cards: Vec<u32>,
	hand_type: u32,
}

impl Hand {
	fn new(cards: &str, map: &HashMap<char, u32>) -> Self {
		let cards = cards
			.chars()
			.map(|c| *map.get(&c).unwrap())
			.collect::<Vec<_>>();

		let mut counts = HashMap::new();

		for i in 0..cards.len() {
			counts.entry(cards[i]).and_modify(|e| *e += 1).or_insert(1);
		}

		// Add jokers value to the count with the highest value

		let mut jokers = counts.remove(&1).unwrap_or(0);

		let mut counts = counts.values_mut().collect::<Vec<_>>();
		counts.sort();

		// add jokers to the highest value
		if counts.len() > 0 {
			**counts.last_mut().unwrap() += jokers;
		} else {
			counts.push(&mut jokers);
		}

		Self {
			cards,
			hand_type: match counts.as_slice() {
				[1, 1, 1, 1, 1] => 1,
				[1, 1, 1, 2] => 2,
				[1, 2, 2] => 3,
				[1, 1, 3] => 4,
				[2, 3] => 5,
				[1, 4] => 6,
				[5] => 7,
				_ => panic!("Invalid hand"),
			},
		}
	}
	fn hand_type(&self) -> u32 {
		// 1 = high card
		// 2 = pair
		// 3 = two pair
		// 4 = three of a kind
		// 5 = full house
		// 6 = four of a kind
		// 7 = five of a kind
		self.hand_type
	}
}

impl PartialEq for Hand {
	fn eq(&self, other: &Self) -> bool {
		self.cards
			.iter()
			.zip(other.cards.iter())
			.all(|(a, b)| *a == *b)
	}
}

impl Eq for Hand {}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let self_type = self.hand_type();
		let other_type = other.hand_type();

		if self_type > other_type {
			return Some(Ordering::Greater);
		} else if self_type < other_type {
			return Some(Ordering::Less);
		}

		for i in 0..self.cards.len() {
			if self.cards[i] > other.cards[i] {
				return Some(Ordering::Greater);
			} else if self.cards[i] < other.cards[i] {
				return Some(Ordering::Less);
			}
		}

		Some(Ordering::Equal)
	}
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

fn solve(input: &str, map: &HashMap<char, u32>) -> Option<u32> {
	Some(
		input
			.lines()
			.map(|line| {
				let (hand, bid) = line.split_once(' ').unwrap();
				let bid = bid.parse::<u32>().unwrap();

				(Hand::new(hand, map), bid)
			})
			.sorted_by_key(|hand| hand.0.clone())
			.fold((1, 0), |(i, acc), f| (i + 1, acc + f.1 * i))
			.1,
	)
}

pub fn part_one(input: &str) -> Option<u32> {
	let map: HashMap<char, u32> = [
		('A', 14),
		('K', 13),
		('Q', 12),
		('J', 11),
		('T', 10),
		('9', 9),
		('8', 8),
		('7', 7),
		('6', 6),
		('5', 5),
		('4', 4),
		('3', 3),
		('2', 2),
	]
	.iter()
	.cloned()
	.collect();

	solve(input, &map)
}

pub fn part_two(input: &str) -> Option<u32> {
	let map: HashMap<char, u32> = [
		('A', 14),
		('K', 13),
		('Q', 12),
		('T', 10),
		('9', 9),
		('8', 8),
		('7', 7),
		('6', 6),
		('5', 5),
		('4', 4),
		('3', 3),
		('2', 2),
		('J', 1),
	]
	.iter()
	.cloned()
	.collect();

	solve(input, &map)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(6440));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(5905));
	}
}
