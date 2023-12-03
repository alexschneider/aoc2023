use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(3);

type Grid = Vec<Vec<char>>;
type Symbols = HashMap<(usize, usize), char>;

fn numbers_iter(grid: Grid) -> impl Iterator<Item = (usize, usize, String)> {
    let mut it = vec![];
    for (y, row) in grid.iter().enumerate() {
        let mut current_num = vec![];
        for (x, ch) in row.iter().enumerate() {
            if ch.is_numeric() {
                current_num.push(*ch);
            } else if !current_num.is_empty() {
                let num = current_num.iter().collect::<String>();
                it.push((x - num.len(), y, num));
                current_num = vec![];
            }
        }
        if !current_num.is_empty() {
            let num = current_num.iter().collect::<String>();
            it.push((row.len() - num.len(), y, num));
        }
    }
    it.into_iter()
}

fn symbol_dict(grid: &Grid) -> Symbols {
    let mut symbol_dict: Symbols = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if !ch.is_numeric() && *ch != '.' {
                print!("{}", ch);
                symbol_dict.insert((x, y), *ch);
            }
        }
    }
    println!("end");
    symbol_dict
}

fn parse_input(input: &str) -> (Grid, Symbols) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let symbols = symbol_dict(&grid);
    (grid, symbols)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, symbols) = parse_input(input);
    let mut ans = 0;
    for (x, y, num) in numbers_iter(grid) {
        let mut found_num = 0;
        for sy in y.saturating_sub(1)..y + 2 {
            for sx in x.saturating_sub(1)..x + num.len() + 1 {
                if symbols.contains_key(&(sx, sy)) {
                    found_num = num.parse().unwrap();
                }
            }
        }
        ans += found_num;
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, symbols) = parse_input(input);
    let mut gear_symbols: HashMap<(usize, usize), Vec<u32>> = symbols
        .iter()
        .filter(|(_, ch)| **ch == '*')
        .map(|((x, y), _)| ((*x, *y), vec![]))
        .collect();
    for (x, y, num) in numbers_iter(grid) {
        for sy in y.saturating_sub(1)..y + 2 {
            for sx in x.saturating_sub(1)..x + num.len() + 1 {
                if let Some(gear) = gear_symbols.get_mut(&(sx, sy)) {
                    gear.push(num.parse().unwrap());
                }
            }
        }
    }
    gear_symbols
        .iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, nums)| nums.iter().product::<u32>())
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}