use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pipe {
    ptype: PipeType,
    counting: bool,
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]

enum PipeType {
    Vertical,   // |
    Horizontal, // -
    NEBend,     // F
    NWBend,     // 7
    SWBend,     // J
    SEBend,     // L
    Ground,     // .
    Starting,   // S
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn coordinates(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    fn flood_coordinates(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Direction::Up => (x, y - 5),
            Direction::Down => (x, y + 5),
            Direction::Left => (x - 5, y),
            Direction::Right => (x + 5, y),
        }
    }
}

impl Pipe {
    fn next_direction(&self, from_direction: &Direction) -> Option<Direction> {
        match self.ptype {
            PipeType::Vertical => match from_direction {
                Direction::Up => Some(Direction::Up),
                Direction::Down => Some(Direction::Down),
                _ => None,
            },
            PipeType::Horizontal => match from_direction {
                Direction::Left => Some(Direction::Left),
                Direction::Right => Some(Direction::Right),
                _ => None,
            },
            PipeType::NEBend => match from_direction {
                Direction::Up => Some(Direction::Right),
                Direction::Left => Some(Direction::Down),
                _ => None,
            },
            PipeType::NWBend => match from_direction {
                Direction::Up => Some(Direction::Left),
                Direction::Right => Some(Direction::Down),
                _ => None,
            },
            PipeType::SWBend => match from_direction {
                Direction::Down => Some(Direction::Left),
                Direction::Right => Some(Direction::Up),
                _ => None,
            },
            PipeType::SEBend => match from_direction {
                Direction::Down => Some(Direction::Right),
                Direction::Left => Some(Direction::Up),
                _ => None,
            },
            PipeType::Ground => None,
            _ => unreachable!("Starting pipe should never be encountered"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (idx, _) = traverse_maze(input);
    Some(idx + 1)
}

fn traverse_maze(input: &str) -> (u32, HashSet<(usize, usize)>) {
    let mut starting = (0, 0);
    let maze = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '|' => Pipe {
                        ptype: PipeType::Vertical,
                        counting: true,
                    },
                    '-' => Pipe {
                        ptype: PipeType::Horizontal,
                        counting: true,
                    },
                    'F' => Pipe {
                        ptype: PipeType::NEBend,
                        counting: true,
                    },
                    '7' => Pipe {
                        ptype: PipeType::NWBend,
                        counting: true,
                    },
                    'J' => Pipe {
                        ptype: PipeType::SWBend,
                        counting: true,
                    },
                    'L' => Pipe {
                        ptype: PipeType::SEBend,
                        counting: true,
                    },
                    '.' => Pipe {
                        ptype: PipeType::Ground,
                        counting: true,
                    },
                    'S' => {
                        starting = (x, y);
                        Pipe {
                            ptype: PipeType::Starting,
                            counting: true,
                        }
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut coordinate_map = HashMap::new();
    let mut current_directions = [
        Direction::Up,
        Direction::Down,
        // Direction::Left,
        // Direction::Right,
    ]
    .iter()
    .filter_map(|dir| {
        if (starting.0 == 0 && *dir == Direction::Left)
            || (starting.1 == 0 && *dir == Direction::Up)
        {
            return None;
        }
        let new_coords = dir.coordinates(starting.0, starting.1);
        if new_coords.0 < maze[0].len() && new_coords.1 < maze.len() {
            let new_dir = maze[new_coords.1][new_coords.0].next_direction(dir)?;
            coordinate_map.insert(new_coords, 1);
            Some((new_dir, new_coords))
        } else {
            None
        }
    })
    .collect::<Vec<_>>();
    let mut idx: u32 = 0;
    loop {
        current_directions = current_directions
            .iter()
            .filter_map(|(dir, (x, y))| {
                let new_coords = dir.coordinates(*x, *y);
                let new_dir = maze[new_coords.1][new_coords.0].next_direction(dir)?;
                *coordinate_map.entry(new_coords).or_insert(0) += 1;
                Some((new_dir, new_coords))
            })
            .collect::<Vec<_>>();
        idx += 1;
        if coordinate_map.values().any(|&v| v > 1) {
            break;
        }
    }
    coordinate_map.entry(starting).or_insert(1);
    (idx, coordinate_map.keys().cloned().collect())
}

// fn is_contained(maze: &Vec<Vec<Pipe>>, (x, y): (usize, usize)) -> bool {
//     let (mut current_x, mut current_y) = (x, y);
//     let mut num_pipes = 0;
//     while current_x > 0 {
//         current_x -= 1;
//         match maze[current_y][current_x] {
//             Pipe::Vertical
//             // | Pipe::NEBend
//             // | Pipe::NWBend
//             // | Pipe::SWBend
//             // | Pipe::SEBend
//             | Pipe::Starting => {
//                 num_pipes += 1;
//             }
//             _ => continue,
//         }
//     }
//     num_pipes % 2 == 1
// }

fn expand_maze(maze: &Vec<Vec<Pipe>>) -> HashMap<(usize, usize), Pipe> {
    let mut new_maze = HashMap::new();
    for y in (0..(maze.len() * 10)).step_by(5) {
        for x in (0..(maze[0].len() * 10)).step_by(5) {
            let pipe = if x % 10 == 0 && y % 10 == 0 {
                maze[y / 10][x / 10]
            } else {
                let pipe_type = if y % 10 == 0 {
                    match maze[y / 10][(x - 5) / 10].ptype {
                        PipeType::Horizontal
                        | PipeType::NEBend
                        | PipeType::SEBend
                        | PipeType::Starting => PipeType::Horizontal,
                        _ => PipeType::Ground,
                    }
                } else if x % 10 == 0 {
                    match maze[(y - 5) / 10][x / 10].ptype {
                        PipeType::Vertical
                        | PipeType::NEBend
                        | PipeType::NWBend
                        | PipeType::Starting => PipeType::Vertical,
                        _ => PipeType::Ground,
                    }
                } else {
                    PipeType::Ground
                };
                Pipe {
                    ptype: pipe_type,
                    counting: false,
                }
            };

            new_maze.insert((x, y), pipe);
        }
    }
    print_maze(&new_maze);
    new_maze
}

fn generate_flood_fill(
    maze: &Vec<Vec<Pipe>>,
    coordinates: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut flood_fill = HashSet::new();

    fn try_fill(
        (x, y): (isize, isize),
        (max_x, max_y): (usize, usize),
        maze: &HashMap<(usize, usize), Pipe>,
        flood_fill: &mut HashSet<(isize, isize)>,
        coordinates: &HashSet<(usize, usize)>,
    ) {
        if flood_fill.contains(&(x, y))
            || x > max_x as isize
            || y > max_y as isize
            || x < -5
            || y < -5
        {
            return;
        }

        // if current_pipe != Pipe::Ground && current_pipe != Pipe::NonCountingGround {
        if x % 10 == 0 && y % 10 == 0 && coordinates.contains(&(x as usize / 10, y as usize / 10)) {
            return;
        }

        let current_pipe = if x < 0 || y < 0 {
            Pipe {
                ptype: PipeType::Ground,
                counting: false,
            }
        } else {
            *maze.get(&(x as usize, y as usize)).unwrap_or(&Pipe {
                ptype: PipeType::Ground,
                counting: false,
            })
        };

        if !current_pipe.counting && current_pipe.ptype != PipeType::Ground {
            return;
        }

        flood_fill.insert((x, y));
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            try_fill(
                direction.flood_coordinates(x, y),
                (max_x, max_y),
                maze,
                flood_fill,
                coordinates,
            );
        }
    }

    // let hash_maze: HashMap<(usize, usize), Pipe> = maze
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(y, row)| {
    //         row.iter()
    //             .enumerate()
    //             .map(|(x, &p)| ((x, y), p))
    //             .collect_vec()
    //     })
    //     .collect();

    let max_x = maze[0].len() * 10;
    let max_y = maze.len() * 10;

    try_fill(
        (-5, -5),
        (max_x, max_y),
        &expand_maze(maze),
        &mut flood_fill,
        coordinates,
    );
    flood_fill
        .iter()
        .filter_map(|(x, y)| {
            if x % 10 == 0 && y % 10 == 0 {
                Some((*x as usize / 10, *y as usize / 10))
            } else {
                None
            }
        })
        // .inspect(|f| println!("f: {:?}", f))
        .collect()
}

fn print_maze(maze: &HashMap<(usize, usize), Pipe>) {
    let max_x = maze.keys().map(|(x, _)| x).max().unwrap();
    let max_y = maze.keys().map(|(_, y)| y).max().unwrap();
    for y in (0..=*max_y).step_by(5) {
        for x in (0..=*max_x).step_by(5) {
            let pipe = maze.get(&(x, y)).unwrap_or(&Pipe {
                ptype: PipeType::Ground,
                counting: false,
            });
            match pipe.ptype {
                PipeType::Vertical => print!("│"),
                PipeType::Horizontal => print!("─"),
                PipeType::NEBend => print!("┌"),
                PipeType::NWBend => print!("┐"),
                PipeType::SWBend => print!("┘"),
                PipeType::SEBend => print!("└"),
                PipeType::Ground => {
                    if pipe.counting {
                        print!(".")
                    } else {
                        print!("*")
                    }
                }
                PipeType::Starting => print!("S"),
            }
        }
        println!();
    }
}

fn print_maze_vec(maze: &Vec<Vec<Pipe>>) {
    for line in maze {
        for pipe in line {
            match pipe.ptype {
                PipeType::Vertical => print!("│"),
                PipeType::Horizontal => print!("─"),
                PipeType::NEBend => print!("┌"),
                PipeType::NWBend => print!("┐"),
                PipeType::SWBend => print!("┘"),
                PipeType::SEBend => print!("└"),
                PipeType::Ground => {
                    if pipe.counting {
                        print!(".")
                    } else {
                        print!("*")
                    }
                }
                PipeType::Starting => print!("S"),
            }
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut starting = (0, 0);
    let maze = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let ptype = match c {
                        '|' => PipeType::Vertical,
                        '-' => PipeType::Horizontal,
                        'F' => PipeType::NEBend,
                        '7' => PipeType::NWBend,
                        'J' => PipeType::SWBend,
                        'L' => PipeType::SEBend,
                        '.' => PipeType::Ground,
                        'S' => {
                            starting = (x, y);
                            PipeType::Starting
                        }
                        _ => unreachable!(),
                    };
                    Pipe {
                        ptype,
                        counting: true,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (_, maze_coordinates) = traverse_maze(input);
    maze_coordinates.iter().for_each(|f| println!("{:?}", f));
    let flood_fill = generate_flood_fill(&maze, &maze_coordinates);
    print_maze_vec(&maze);
    Some(
        maze.iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, _)| (x, y))
                    .collect::<Vec<_>>()
            })
            .filter(|(x, y)| {
                !flood_fill.contains(&(*x, *y)) && !maze_coordinates.contains(&(*x, *y))
            })
            // .inspect(|c: &(usize, usize)| println!("{:?}", c))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
