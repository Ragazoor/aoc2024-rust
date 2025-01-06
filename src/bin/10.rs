use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
struct Tile {
    height: u32,
}

fn parse_input(input: &str) -> HashMap<(i32, i32), Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    (x as i32, y as i32),
                    Tile {
                        height: c.to_digit(10).unwrap(),
                    },
                )
            })
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u64> {
    let tile_map = parse_input(input);
    let result = tile_map
        .iter()
        .map(|(pos, tile)| {
            if tile.height == 0 {
                get_trail_head_score(&tile_map, *pos)
            } else {
                0
            }
        })
        .sum::<u64>();

    Some(result)
}

fn get_trail_head_score(tile_map: &HashMap<(i32, i32), Tile>, pos: (i32, i32)) -> u64 {
    get_trail_head_peaks(tile_map, pos).len() as u64
}

fn get_trail_head_peaks(
    tile_map: &HashMap<(i32, i32), Tile>,
    pos: (i32, i32),
) -> HashSet<(i32, i32)> {
    let current_tile = tile_map.get(&pos).unwrap();
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    dirs.iter()
        .flat_map(|(dx, dy)| {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if let Some(tile) = tile_map.get(&new_pos) {
                if tile.height == current_tile.height + 1 && tile.height < 9 {
                    get_trail_head_peaks(tile_map, new_pos)
                } else if tile.height == current_tile.height + 1 && tile.height == 9 {
                    let mut new_peak = HashSet::new();
                    new_peak.insert(new_pos);
                    new_peak
                } else {
                    HashSet::new()
                }
            } else {
                HashSet::new()
            }
        })
        .collect()
}

fn print_tile_map_with_pos(tile_map: &HashMap<(i32, i32), Tile>, pos: (i32, i32)) {
    let min_x = 0;
    let max_x = *tile_map.keys().map(|(x, _)| x).max().unwrap();
    let min_y = 0;
    let max_y = *tile_map.keys().map(|(_, y)| y).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if pos == (x, y) {
                print!("X");
            } else {
                match tile_map.get(&(x, y)) {
                    Some(tile) => print!("{}", tile.height),
                    None => panic!("No tile found at {:?}", (x, y)),
                }
            }
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let tile_map = parse_input(input);
    let result = tile_map
        .iter()
        .map(|(pos, tile)| {
            if tile.height == 0 {
                get_num_paths(&tile_map, *pos)
            } else {
                0
            }
        })
        .sum::<u64>();

    Some(result)
}

fn get_num_paths(tile_map: &HashMap<(i32, i32), Tile>, pos: (i32, i32)) -> u64 {
    let current_tile = tile_map.get(&pos).unwrap();
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    dirs.iter()
        .map(|(dx, dy)| {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if let Some(tile) = tile_map.get(&new_pos) {
                if tile.height == current_tile.height + 1 && tile.height < 9 {
                    get_num_paths(tile_map, new_pos)
                } else if tile.height == current_tile.height + 1 && tile.height == 9 {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_part_one() {
        let input = "0123\n\
                           1234\n\
                           8765\n\
                           9876";

        let result = part_one(input);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_three_part_one() {
        let input = "1091911\n\
                           2991811\n\
                           3111711\n\
                           4567654\n\
                           1118113\n\
                           1119392\n\
                           1111201";

        let result = part_one(input);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
