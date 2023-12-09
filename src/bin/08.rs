use std::collections::HashMap;

use num::Integer;

advent_of_code::solution!(8);

type NodeMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_input(input: &str) -> (impl Iterator<Item = char> + Clone + '_, NodeMap) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();

    lines.next();
    let nodes = lines
        .filter_map(|line| {
            let (node, leafs) = line.split_once(" = ")?;
            let (l_leaf, r_leaf) = leafs.trim_matches(&['(', ')'][..]).split_once(", ")?;
            Some((node, (l_leaf, r_leaf)))
        })
        .collect::<NodeMap>();
    (instructions, nodes)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut instructions, nodes) = parse_input(input);
    let mut steps = 0;
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        let (l_leaf, r_leaf) = nodes.get(current_node)?;
        let next_instruction = instructions.next()?;
        current_node = if next_instruction == 'L' {
            l_leaf
        } else {
            r_leaf
        };
        steps += 1;
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u128> {
    let (instructions, nodes) = parse_input(input);
    let starting_nodes = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect::<Vec<&str>>();

    let starting_node_counts = starting_nodes.iter().filter_map(|&s| {
        let mut current_instructions = instructions.clone();
        let mut current_node = s;
        let mut steps = 0;
        while !current_node.ends_with('Z') {
            let (l_leaf, r_leaf) = nodes.get(current_node)?;
            let next_instruction = current_instructions.next()?;
            current_node = if next_instruction == 'L' {
                l_leaf
            } else {
                r_leaf
            };
            steps += 1;
        }
        Some(steps)
    });

    Some(starting_node_counts.fold(1, |acc, x| acc.lcm(&x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
