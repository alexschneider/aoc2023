use core::time;

use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (times, distances) = input
        .lines()
        .map(str::split_ascii_whitespace)
        .map(|i| i.skip(1))
        .map(|i| i.map(|s| s.parse::<u32>().unwrap()))
        .collect_tuple()
        .unwrap();
    let mut ans = 1;
    for (time, distance) in times.zip(distances) {
        let mut successes = 0;
        for i in 0..time {
            if i * (time - i) > distance {
                successes += 1;
            }
        }
        ans *= successes;
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (time, distance) = input
        .lines()
        .map(str::split_ascii_whitespace)
        .map(|i| {
            i.skip(1)
                .fold(String::new(), |mut acc, s| {
                    acc.push_str(s);
                    acc
                })
                .parse::<u128>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    let mut successes = 0;
    for i in 0..time {
        if i * (time - i) > distance {
            successes += 1;
        }
    }
    Some(successes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
