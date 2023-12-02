use itertools::Itertools;

advent_of_code::solution!(2);

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

fn rounds(line: &str) -> impl Iterator<Item = &str> {
    line.split(':').nth(1).unwrap_or("").split(';')
}

fn parse_nums_color(cube: &str) -> (u32, &str) {
    let (num, color) = cube.trim().split(' ').tuple_windows().next().unwrap();
    let num = num.parse::<u32>().unwrap();
    (num, color)
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;
    for (id, line) in input.lines().enumerate() {
        let mut possible: bool = true;
        for round in rounds(line) {
            for cube in round.split(',') {
                let (num, color) = parse_nums_color(cube);
                let possible_round = match color {
                    "red" => num <= RED_CUBES,
                    "green" => num <= GREEN_CUBES,
                    "blue" => num <= BLUE_CUBES,
                    _ => panic!("Unexpected input: {}", cube),
                };
                if !possible_round {
                    possible = false
                }
            }
        }
        if possible {
            ans += (id + 1) as u32;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;
    for line in input.lines() {
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
        for round in rounds(line) {
            for cube in round.split(',') {
                let (num, color) = parse_nums_color(cube);
                match color {
                    "red" => {
                        if num > min_red {
                            min_red = num
                        }
                    }
                    "green" => {
                        if num > min_green {
                            min_green = num
                        }
                    }
                    "blue" => {
                        if num > min_blue {
                            min_blue = num
                        }
                    }
                    _ => panic!("Unexpected input: {}", cube),
                };
            }
        }
        ans += min_red * min_blue * min_green;
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
