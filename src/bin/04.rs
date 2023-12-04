use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(4);

fn find_winners(line: &str) -> usize {
    let card = line.split(':').nth(1).unwrap().trim();

    let (winning, mine) = card.split('|').collect_tuple().unwrap();
    let winning = winning
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<HashSet<&str>>();

    let mine = mine
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<HashSet<&str>>();

    winning.intersection(&mine).count()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut accum = 0;
    for line in input.lines() {
        let num_winners = find_winners(line);
        if num_winners > 0 {
            accum += 1 << (num_winners - 1);
        }
    }
    Some(accum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut next_copies: HashMap<usize, usize> = HashMap::new();
    for (card_num, line) in input.lines().enumerate() {
        let current_copies = *next_copies.entry(card_num).or_insert(1);

        let num_winners = find_winners(line);
        for card in card_num + 1..card_num + num_winners + 1 {
            *next_copies.entry(card).or_insert(1) += current_copies;
        }
    }
    next_copies.values().sum::<usize>().try_into().ok()
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
