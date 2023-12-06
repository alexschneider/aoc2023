use itertools::Itertools;

advent_of_code::solution!(5);

type Mapping = Vec<Vec<(usize, usize, usize)>>;

fn get_maps_and_seeds(input: &str) -> (Mapping, Vec<usize>) {
    let mut lines = input.lines();
    let mut maps = vec![];
    let seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .flat_map(str::parse::<usize>)
        .collect_vec();
    lines.next();
    while let Some(mut line) = lines.next() {
        if line.contains("map") {
            continue;
        }
        let mut current_mapping = vec![];
        loop {
            let (destination_start, source_start, length) =
                line.split(' ').collect_tuple().unwrap();

            current_mapping.push((
                destination_start.parse::<usize>().unwrap(),
                source_start.parse::<usize>().unwrap(),
                length.parse::<usize>().unwrap(),
            ));
            line = lines.next().unwrap_or_default();
            if line.is_empty() {
                break;
            }
        }
        maps.push(current_mapping);
    }
    (maps, seeds)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (maps, seeds) = get_maps_and_seeds(input);
    let mut lowest = u32::MAX;
    for mut seed in seeds {
        for map in &maps {
            for &(destination_start, source_start, length) in map {
                if seed >= source_start && seed < source_start + length {
                    let offset = seed - source_start;
                    seed = destination_start + offset;

                    break;
                }
            }
        }
        lowest = lowest.min(seed as u32);
    }

    Some(lowest)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (maps, seeds) = get_maps_and_seeds(input);
    let new_seeds: Vec<(usize, usize)> = seeds
        .chunks(2)
        .sorted_by(|c0, c1| c0[0].cmp(&c1[0]))
        .map(|c| (c[0], c[1]))
        .collect_vec();

    let mut i = 1;
    loop {
        let mut seed = i;
        for map in maps.iter().rev() {
            for &(destination_start, source_start, length) in map {
                if seed >= destination_start && seed < destination_start + length {
                    let offset = seed - destination_start;
                    seed = source_start + offset;

                    break;
                }
            }
        }
        if check_contains(seed, &new_seeds) {
            break;
        } else {
            i += 1;
        }
    }

    Some(i as u32)
}

fn check_contains(num: usize, seeds: &Vec<(usize, usize)>) -> bool {
    for (seed_start, seed_len) in seeds {
        if num >= *seed_start && num < seed_start + seed_len {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
