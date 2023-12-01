advent_of_code::solution!(1);

fn as_number(c: char) -> Option<u32> {
    if c.is_ascii_digit() {
        Some(c as u32 - '0' as u32)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(as_number);
            let last = line.chars().rev().find_map(as_number);

            match (first, last) {
                (Some(f), Some(l)) => Some(f * 10 + l),
                _ => None,
            }
        })
        .sum()
}

static NUMBERS: [(&str, u32); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn begins_with(line: &str) -> Option<u32> {
    for (word, number) in NUMBERS.iter() {
        if line.starts_with(word) {
            return Some(*number);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let first = (0..line.len()).find_map(|i| begins_with(&line[i..]));
            let last = (1..=line.len()).find_map(|i| begins_with(&line[line.len() - i..]));

            match (first, last) {
                (Some(f), Some(l)) => Some(f * 10 + l),
                _ => None,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
