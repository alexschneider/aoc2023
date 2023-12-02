use itertools::Itertools;
use std::iter;

advent_of_code::solution!(1);

fn find_num(line: String) -> u32 {
    let full_num = line.chars().filter(|c| c.is_numeric()).collect_vec();
    full_num
        .iter()
        .take(1)
        .chain(iter::once(full_num.last().unwrap()))
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| find_num(line.to_string()))
        .sum::<u32>()
        .into()
}

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            (0..line.len())
                .map(|idx| {
                    if line.chars().nth(idx).unwrap().is_numeric() {
                        line[idx..idx + 1].to_string()
                    } else {
                        WORDS
                            .iter()
                            .enumerate()
                            .find_map(|(i, word)| {
                                if line[idx..].starts_with(word) {
                                    Some((i + 1).to_string())
                                } else {
                                    None
                                }
                            })
                            .unwrap_or("".to_string())
                    }
                })
                .collect()
        })
        .map(find_num)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
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
