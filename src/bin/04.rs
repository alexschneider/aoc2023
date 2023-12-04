use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

fn find_winners(line: &str) -> usize {
    let card = line.split(':').nth(1).unwrap().trim();
    let winning = card
        .split('|')
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<HashSet<&str>>();
    let mine = card
        .split('|')
        .nth(1)
        .unwrap()
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
            accum += 2_u32.pow(num_winners as u32 - 1);
        }
    }
    Some(accum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut next_copies: HashMap<usize, usize> = HashMap::new();
    for (card_num, line) in input.lines().enumerate() {
        let current_copies = *next_copies.get(&card_num).unwrap_or(&1);
        next_copies.insert(card_num, current_copies);

        let num_winners = find_winners(line);
        for card in card_num + 1..card_num + num_winners + 1 {
            next_copies.insert(card, next_copies.get(&card).unwrap_or(&1) + current_copies);
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
