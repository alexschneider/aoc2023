use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut galaxies = vec![];
    for row in input.lines() {
        galaxies.push(row.to_string());
        if !row.contains('#') {
            galaxies.push(row.to_string());
        }
    }
    let mut galaxy_len = galaxies[0].len();
    let mut col = 0;
    while col < galaxy_len {
        let mut found_galaxy = false;
        for row in &galaxies {
            let galaxy = row.chars().nth(col).unwrap();
            if galaxy == '#' {
                found_galaxy = true;
            }
        }
        if !found_galaxy {
            for row in galaxies.iter_mut() {
                row.insert(col, '.');
            }
            col += 1;
            galaxy_len += 1;
        }
        col += 1;
    }

    let mut numbered_galaxies = vec![];

    for (y, row) in galaxies.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch != '.' {
                numbered_galaxies.push((x, y));
            }
        }
    }

    let mut distances = vec![];

    for (i, (x, y)) in numbered_galaxies.iter().enumerate() {
        for (j, (x2, y2)) in numbered_galaxies[i + 1..].iter().enumerate() {
            println!(
                "Galaxy i: {} j: {} dist: {}",
                i,
                j,
                x.abs_diff(*x2) + y.abs_diff(*y2)
            );
            distances.push(x.abs_diff(*x2) + y.abs_diff(*y2));
        }
    }

    Some(distances.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut galaxies = vec![];
    let mut empty_rows = HashSet::new();
    let mut empty_cols = HashSet::new();
    for (i, row) in input.lines().enumerate() {
        galaxies.push(row.to_string());
        if !row.contains('#') {
            empty_rows.insert(i);
        }
    }
    let mut galaxy_len = galaxies[0].len();
    let mut col = 0;
    while col < galaxy_len {
        let mut found_galaxy = false;
        for row in &galaxies {
            let galaxy = row.chars().nth(col).unwrap();
            if galaxy == '#' {
                found_galaxy = true;
            }
        }
        if !found_galaxy {
            empty_cols.insert(col);
        }
        col += 1;
    }

    let mut numbered_galaxies = vec![];

    for (y, row) in galaxies.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch != '.' {
                numbered_galaxies.push((x, y));
            }
        }
    }

    let mut distances = vec![];
    println!("Part 2");
    for (i, (x, y)) in numbered_galaxies.iter().enumerate() {
        for (j, (x2, y2)) in numbered_galaxies[i + 1..].iter().enumerate() {
            let mut num_empty_cols = 0;
            if i == 0 && j == 1 {
                println!("here");
            }

            let min_x = x.min(x2);
            let max_x = x.max(x2);
            for tx in *min_x..*max_x {
                if empty_cols.contains(&tx) {
                    num_empty_cols += 1;
                }
            }
            let mut num_empty_rows = 0;
            for ty in *y..*y2 {
                if empty_rows.contains(&ty) {
                    num_empty_rows += 1;
                }
            }
            let expansion = 1_000_000 - 1;
            distances.push(
                x.abs_diff(*x2)
                    + y.abs_diff(*y2)
                    + num_empty_cols * expansion
                    + num_empty_rows * expansion,
            );
        }
    }

    Some(distances.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }
}
