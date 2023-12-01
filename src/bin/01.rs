use itertools::Itertools;

advent_of_code::solution!(1);

fn find_num(line: String) -> u32 {
    let full_num = line.chars().filter(|c| c.is_numeric()).collect_vec();
    let mut num = full_num.first().unwrap().to_string();
    num.push(*full_num.last().unwrap());
    num.parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(|line| find_num(line.to_string())).sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().map(|line| {
        let mut nums: Vec<String> = vec![];
        for idx in 0..line.len() {
            if line.chars().nth(idx).unwrap().is_numeric() {
                nums.push(line[idx..idx+1].to_string());
            } else {
                nums.push(if line[idx..].starts_with("one") {
                    "1"
                } else if line[idx..].starts_with("two") {
                    "2"
                } else if line[idx..].starts_with ("three") {
                    "3"
                } else if line[idx..].starts_with ("four") {
                    "4"
                } else if line[idx..].starts_with ("five") {
                    "5"
                } else if line[idx..].starts_with ("six") {
                    "6"
                } else if line[idx..].starts_with ("seven") {
                    "7"
                } else if line[idx..].starts_with ("eight") {
                    "8"
                } else if line[idx..].starts_with ("nine") {
                    "9"
                } else if line[idx..].starts_with ("zero") {
                    "0"
                } else {
                    ""
                }.to_string())
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
