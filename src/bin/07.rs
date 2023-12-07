use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let ans = input
        .lines()
        .filter_map(|l| l.split_whitespace().collect_tuple::<(&str, &str)>())
        .sorted_by(|a, b| cmp_cards_1(a.0, b.0))
        .enumerate()
        .map(|(i, (_, bid))| {
            let bid = bid.parse::<u32>().unwrap();
            bid * (i + 1) as u32
        });
    Some(ans.sum())
}

fn count_cards(hand: &str) -> HashMap<char, u32> {
    let mut counter = HashMap::new();
    for c in hand.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }
    counter
}

fn cmp_cards_1(a: &str, b: &str) -> std::cmp::Ordering {
    let counter_a = count_cards(a);
    let counter_b = count_cards(b);
    let max_a = counter_a.values().max();
    let max_b = counter_b.values().max();
    // check to see if one has more matches than the other
    if max_a != max_b {
        return max_a.cmp(&max_b);
    }

    // check to see if there's a full house or 2pair in one card and not the other
    let second_match_a = counter_a.values().sorted().rev().nth(1);
    let second_match_b = counter_b.values().sorted().rev().nth(1);
    if second_match_a != second_match_b
        && (second_match_a == Some(&2) || second_match_b == Some(&2))
    {
        return second_match_a.cmp(&second_match_b);
    }

    // check which match has the higher card(s) with stupid AoC card ordering rule
    for (ch_a, ch_b) in a.chars().zip(b.chars()) {
        let val_a = card_value_1(&ch_a);
        let val_b = card_value_1(&ch_b);
        if val_a != val_b {
            return val_a.cmp(&val_b);
        }
    }

    Ordering::Equal
}

fn card_value_1(card: &char) -> u32 {
    match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap(),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let ans = input
        .lines()
        .filter_map(|l| l.split_whitespace().collect_tuple::<(&str, &str)>())
        .sorted_by(|a, b| cmp_cards_2(a.0, b.0))
        .enumerate()
        .map(|(i, (_, bid))| {
            let bid = bid.parse::<u32>().unwrap();
            bid * (i + 1) as u32
        });
    Some(ans.sum())
}

fn cmp_cards_2(a: &str, b: &str) -> std::cmp::Ordering {
    let counter_a = count_cards(a);
    let counter_b = count_cards(b);
    let joker_a = counter_a.get(&'J').unwrap_or(&0);
    let joker_b = counter_b.get(&'J').unwrap_or(&0);
    let mut joker_counter_a = counter_a.clone();
    let mut joker_counter_b = counter_b.clone();
    joker_counter_a.remove(&'J');
    joker_counter_b.remove(&'J');
    let max_a = joker_counter_a.values().max().unwrap_or(&0) + joker_a;
    let max_b = joker_counter_b.values().max().unwrap_or(&0) + joker_b;

    // check to see if one has more matches than the other
    if max_a != max_b {
        return max_a.cmp(&max_b);
    }

    // check to see if there's a full house or 2pair in one card and not the other
    let second_match_a = joker_counter_a.values().sorted().rev().nth(1);
    let second_match_b = joker_counter_b.values().sorted().rev().nth(1);
    if second_match_a != second_match_b
        && (second_match_a == Some(&2) || second_match_b == Some(&2))
    {
        return second_match_a.cmp(&second_match_b);
    }

    // check which match has the higher card(s) with stupid AoC card ordering rule
    for (ch_a, ch_b) in a.chars().zip(b.chars()) {
        let val_a = card_value_2(&ch_a);
        let val_b = card_value_2(&ch_b);
        if val_a != val_b {
            return val_a.cmp(&val_b);
        }
    }

    Ordering::Equal
}

fn card_value_2(card: &char) -> u32 {
    match card {
        'T' => 10,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap(),
    }
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
