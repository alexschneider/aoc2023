use std::iter;
use itertools::Itertools;

advent_of_code::solution!(1);

fn find_num(line: String) -> u32 {
    let full_num = line.chars().filter(|c| c.is_numeric()).collect_vec();
    full_num.iter().take(1).chain(iter::once(full_num.last().unwrap())).collect::<String>().parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(|line| find_num(line.to_string())).sum::<u32>().into()
}

const WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().map(|line| {
        let mut nums: Vec<String> = vec![];
        for idx in 0..line.len() {
            if line.chars().nth(idx).unwrap().is_numeric() {
                nums.push(line[idx..idx+1].to_string());
            } else {
                for (i, word) in WORDS.iter().enumerate() {
                    if line[idx..].starts_with(word) {
                        nums.push((i + 1).to_string());
                        break;
                    }
                }
            }
        }
        nums.concat()
    }).map(find_num).sum::<u32>().into()
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
