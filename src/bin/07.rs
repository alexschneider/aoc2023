use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

advent_of_code::solution!(7);

struct Counters {
    counter: HashMap<char, u32>,
    max: u32,
    joker_counter: HashMap<char, u32>,
    max_with_joker: u32,
}

impl Counters {
    fn new(hand: &str) -> Self {
        let counter = count_cards(hand);
        let max = *counter.values().max().unwrap_or(&0);
        let joker = *counter.get(&'J').unwrap_or(&0);
        let mut joker_counter = counter.clone();
        joker_counter.remove(&'J');
        let max_with_joker = joker_counter.values().max().unwrap_or(&0) + joker;

        Counters {
            counter,
            max,
            joker_counter,
            max_with_joker,
        }
    }
}

fn count_cards(hand: &str) -> HashMap<char, u32> {
    let mut counter = HashMap::new();
    for c in hand.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }
    counter
}

fn second_match_from_counter(counter: HashMap<char, u32>) -> Option<u32> {
    counter.values().sorted().rev().nth(1).copied()
}

fn card_value(card: &char, is_part_2: bool) -> u32 {
    match card {
        'T' => {
            if is_part_2 {
                0
            } else {
                10
            }
        }
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap(),
    }
}

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

fn cmp_cards_1(a: &str, b: &str) -> std::cmp::Ordering {
    let counter_a = Counters::new(a);
    let counter_b = Counters::new(b);
    // check to see if one has more matches than the other
    if counter_a.max != counter_b.max {
        return counter_a.max.cmp(&counter_b.max);
    }

    // check to see if there's a full house or 2pair in one card and not the other
    let second_match_a = second_match_from_counter(counter_a.counter);
    let second_match_b = second_match_from_counter(counter_b.counter);
    if second_match_a != second_match_b && (second_match_a == Some(2) || second_match_b == Some(2))
    {
        return second_match_a.cmp(&second_match_b);
    }

    // check which match has the higher card(s) with stupid AoC card ordering rule
    for (ch_a, ch_b) in a.chars().zip(b.chars()) {
        let val_a = card_value(&ch_a, false);
        let val_b = card_value(&ch_b, false);
        if val_a != val_b {
            return val_a.cmp(&val_b);
        }
    }

    Ordering::Equal
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
    let counters_a = Counters::new(a);
    let counters_b = Counters::new(b);

    // check to see if one has more matches than the other
    if counters_a.max_with_joker != counters_b.max_with_joker {
        return counters_a.max_with_joker.cmp(&counters_b.max_with_joker);
    }

    // check to see if there's a full house or 2pair in one card and not the other
    let second_match_a = second_match_from_counter(counters_a.joker_counter);
    let second_match_b = second_match_from_counter(counters_b.joker_counter);
    if second_match_a != second_match_b && (second_match_a == Some(2) || second_match_b == Some(2))
    {
        return second_match_a.cmp(&second_match_b);
    }

    // check which match has the higher card(s) with stupid AoC card ordering rule
    for (ch_a, ch_b) in a.chars().zip(b.chars()) {
        let val_a = card_value(&ch_a, true);
        let val_b = card_value(&ch_b, true);
        if val_a != val_b {
            return val_a.cmp(&val_b);
        }
    }

    Ordering::Equal
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
