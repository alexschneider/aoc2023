use itertools::Itertools;

advent_of_code::solution!(9);

fn parse_differences(input: &str) -> Option<Vec<Vec<i32>>> {
    let mut differences = vec![input
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()];

    while !differences.last()?.iter().all(|&n| n == 0) {
        differences.push(
            differences
                .last()
                .unwrap()
                .iter()
                .tuple_windows::<(_, _)>()
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>(),
        );
    }

    differences.reverse();
    Some(differences)
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut accum = 0;
    for line in input.lines() {
        let mut differences = parse_differences(line)?;
        for i in 0..(differences.len() - 1) {
            if differences[i].iter().all(|&n| n == 0) {
                differences[i].push(0);
            }
            let last = *differences[i].last()?;
            let last_1 = *differences[i + 1].last()?;
            differences[i + 1].push(last_1 + last);
        }
        accum += differences.last()?.last()?;
    }

    Some(accum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut accum = 0;
    for line in input.lines() {
        let mut differences = parse_differences(line)?;

        for i in 0..(differences.len() - 1) {
            if differences[i].iter().all(|&n| n == 0) {
                differences[i].push(0);
            }
            let first = *differences[i].first()?;
            let first_1 = *differences[i + 1].first()?;
            differences[i + 1].insert(0, first_1 - first);
        }
        accum += differences.last()?.first()?;
    }

    Some(accum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
