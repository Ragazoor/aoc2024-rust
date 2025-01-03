use std::{collections::HashSet, hash::Hash};

advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Tile {
    Empty,
    Walked,
    Walked2(Direction),
    GuardTile(Direction),
    Obstacle,
    Obstacle2,
}

#[derive(Copy, Clone, Debug)]
struct Guard {
    pos: (i64, i64),
    dir: Direction,
    dir_diff: (i64, i64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let guard_opt = grid.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, tile)| match tile {
            Tile::GuardTile(dir) => Some(Guard {
                pos: (x as i64, y as i64),
                dir: *dir,
                dir_diff: get_dir_diff(dir),
            }),
            _ => None,
        })
    });
    let guard = match guard_opt {
        Some(guard) => guard,
        None => panic!("No guard found"),
    };
    println!("Guard: {:?}", guard);

    let walked_grid = simulate_walk(grid, guard);
    let walked_tile_count = walked_grid
        .iter()
        .map(|row| row.iter().filter(|tile| **tile == Tile::Walked).count())
        .sum::<usize>();

    println!("Walked grid:");
    print_grid(&walked_grid);
    Some(walked_tile_count as u64)
}

fn get_dir_diff(dir: &Direction) -> (i64, i64) {
    match dir {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

fn print_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid.iter() {
        for tile in row.iter() {
            match tile {
                Tile::Empty => print!("."),
                Tile::Walked => print!("X"),
                Tile::GuardTile(dir) => match dir {
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                    Direction::Right => print!(">"),
                },
                Tile::Walked2(dir) => match dir {
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                    Direction::Right => print!(">"),
                },
                Tile::Obstacle => print!("#"),
                Tile::Obstacle2 => print!("O"),
            }
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    let guard_opt = grid.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, tile)| match tile {
            Tile::GuardTile(dir) => Some(Guard {
                pos: (x as i64, y as i64),
                dir: *dir,
                dir_diff: get_dir_diff(dir),
            }),
            _ => None,
        })
    });
    let mut guard = match guard_opt {
        Some(guard) => guard,
        None => panic!("No guard found"),
    };
    println!("Guard: {:?}", guard);

    let orig_pos = guard.pos;

    let mut obstacle_set: HashSet<(i64, i64)> = HashSet::new();
    let mut visited_states = HashSet::new();
    loop {
        grid[guard.pos.1 as usize][guard.pos.0 as usize] = Tile::Walked2(guard.dir);
        visited_states.insert((guard.pos.0 as u64, guard.pos.1 as u64, guard.dir));
        match get_obstacle_loop_pos(&grid, &guard, &visited_states) {
            Some(obstacle_pos) => {
                obstacle_set.insert(obstacle_pos);
            }
            None => {}
        }
        match get_next_guard(&guard, &grid) {
            Some(new_guard) => {
                grid[new_guard.pos.1 as usize][new_guard.pos.0 as usize] =
                    Tile::GuardTile(new_guard.dir);
                guard = new_guard;
            }

            None => {
                println!("Guard is outside the grid {:?}", guard);
                break;
            }
        }
    }
    obstacle_set.remove(&orig_pos);

    println!("Walked grid:");
    for pos in obstacle_set.iter() {
        grid[pos.1 as usize][pos.0 as usize] = Tile::Obstacle2;
    }
    print_grid(&grid);

    Some(obstacle_set.iter().len() as u64)
}

fn simulate_walk(grid: Vec<Vec<Tile>>, guard: Guard) -> Vec<Vec<Tile>> {
    let mut new_grid = grid.clone();
    let new_guard_opt = get_next_guard(&guard, &new_grid);
    new_grid[guard.pos.1 as usize][guard.pos.0 as usize] = Tile::Walked;
    match new_guard_opt {
        Some(new_guard) => {
            new_grid[new_guard.pos.1 as usize][new_guard.pos.0 as usize] =
                Tile::GuardTile(new_guard.dir);
            simulate_walk(new_grid, new_guard)
        }
        None => {
            print!("Guard is outside the grid {:?}", guard);
            print_grid(&new_grid);
            new_grid
        }
    }
}

fn simulate_walk_2(
    mut grid: Vec<Vec<Tile>>,
    mut visited_states: HashSet<(u64, u64, Direction)>,
    mut guard: Guard,
) -> bool {
    let mut has_loop = false;
    loop {
        grid[guard.pos.1 as usize][guard.pos.0 as usize] = Tile::Walked2(guard.dir);
        match get_next_guard(&guard, &grid) {
            Some(new_guard) => {
                grid[new_guard.pos.1 as usize][new_guard.pos.0 as usize] =
                    Tile::GuardTile(new_guard.dir);

                if visited_states.contains(&(
                    new_guard.pos.0 as u64,
                    new_guard.pos.1 as u64,
                    new_guard.dir,
                )) {
                    has_loop = true;
                    break;
                } else {
                    visited_states.insert((
                        new_guard.pos.0 as u64,
                        new_guard.pos.1 as u64,
                        new_guard.dir,
                    ));
                }
                guard = new_guard;
            }
            None => {
                //print!("Guard is outside the grid {:?}", guard);
                break;
            }
        }
    }
    return has_loop;
}

fn get_obstacle_loop_pos(
    grid: &Vec<Vec<Tile>>,
    guard: &Guard,
    visited_states: &HashSet<(u64, u64, Direction)>,
) -> Option<(i64, i64)> {
    let right_turn_dir = turn_right_dir(&guard.dir);
    let new_guard = Guard {
        pos: guard.pos,
        dir: right_turn_dir,
        dir_diff: get_dir_diff(&right_turn_dir),
    };
    let mut new_grid = grid.clone();

    let obstacle_pos = (
        guard.pos.0 + guard.dir_diff.0,
        guard.pos.1 + guard.dir_diff.1,
    );
    let grid_size = (grid[0].len() as u64, grid.len() as u64);
    if is_inside_grid(obstacle_pos, grid_size) && can_create_obstacle(&grid, obstacle_pos) {
        new_grid[obstacle_pos.1 as usize][obstacle_pos.0 as usize] = Tile::Obstacle2;
        if simulate_walk_2(new_grid, visited_states.clone(), new_guard) {
            Some(obstacle_pos)
        } else {
            None
        }
    } else {
        None
    }
}

fn can_create_obstacle(grid: &Vec<Vec<Tile>>, new_pos: (i64, i64)) -> bool {
    let tile = grid[new_pos.1 as usize][new_pos.0 as usize];
    match tile {
        Tile::Empty => true,
        _ => false,
    }
}

fn turn_right_dir(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_next_guard(guard: &Guard, grid: &Vec<Vec<Tile>>) -> Option<Guard> {
    let grid_size = (grid[0].len() as u64, grid.len() as u64);
    let new_guard_pos = (
        guard.pos.0 + guard.dir_diff.0,
        guard.pos.1 + guard.dir_diff.1,
    );
    if is_inside_grid(new_guard_pos, grid_size) {
        match grid[new_guard_pos.1 as usize][new_guard_pos.0 as usize] {
            Tile::Empty | Tile::Walked | Tile::Walked2(_) => {
                let new_guard = Guard {
                    pos: new_guard_pos,
                    dir: guard.dir,
                    dir_diff: guard.dir_diff,
                };
                Some(new_guard)
            }
            Tile::Obstacle | Tile::Obstacle2 => {
                let new_dir = turn_right_dir(&guard.dir);
                let new_guard = Guard {
                    pos: guard.pos,
                    dir: new_dir,
                    dir_diff: get_dir_diff(&new_dir),
                };
                Some(new_guard)
            }
            tile => {
                print_grid(grid);
                panic!("Invalid tile {:?} on position {:?}", tile, new_guard_pos);
            }
        }
    } else {
        // Guard is outside the grid
        //println!("New guard position is outside the grid {:?}", new_guard_pos);
        None
    }
}

fn is_inside_grid(new_pos: (i64, i64), grid_size: (u64, u64)) -> bool {
    new_pos.0 >= 0
        && new_pos.0 < grid_size.0 as i64
        && new_pos.1 >= 0
        && new_pos.1 < grid_size.1 as i64
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Obstacle,
                    '^' => Tile::GuardTile(Direction::Up),
                    '>' => Tile::GuardTile(Direction::Right),
                    'v' => Tile::GuardTile(Direction::Down),
                    '<' => Tile::GuardTile(Direction::Left),
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_1() {
        let input = ".#.#...\n\
                            #...#..\n\
                            .#.....\n\
                            #.....#\n\
                            .....#.\n\
                            ...^...\n\
                            .......\n";
        let result = part_two(input);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_2() {
        let input = ".......\n\
                            ...##..\n\
                            ......#\n\
                            ...^...\n\
                            .....#.\n\
                            .......\n\
                            .......\n";
        let result = part_two(input);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_3() {
        let input = ".##....\n\
                            #....#.\n\
                            ##..#..\n\
                            ^...#..\n\
                            ...#...\n\
                            .......\n\
                            .......\n";
        let result = part_two(input);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two_4() {
        let input = ".#..\n\
                            #..#\n\
                            ....\n\
                            ^...\n\
                            #...\n\
                            .#..\n";
        let result = part_two(input);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_5() {
        let input = ".#..\n\
                            ....\n\
                            ....\n\
                            #^..\n\
                            .#..\n\
                            ....\n";
        let result = part_two(input);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_is_inside_grid() {
        assert_eq!(is_inside_grid((0, 0), (10, 10)), true);
        assert_eq!(is_inside_grid((10, 10), (10, 10)), false);
        assert_eq!(is_inside_grid((-1, 0), (10, 10)), false);
        assert_eq!(is_inside_grid((0, -1), (10, 10)), false);
    }
}
