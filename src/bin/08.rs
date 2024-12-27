use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Antenna(char),
}

fn _parse_input(input: &str) -> HashMap<(i64, i64), Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    (x as i64, y as i64),
                    match c {
                        '.' => Tile::Empty,
                        c => Tile::Antenna(c),
                    },
                )
            })
        })
        .collect()
}

fn get_grid_size(input: &str) -> (usize, usize) {
    let y = input.lines().count();
    let x = input.lines().next().unwrap().chars().count();
    (x, y)
}

fn get_tile_map(input: &str) -> HashMap<Tile, Vec<(i64, i64)>> {
    let mut tile_map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    match c {
                        '.' => Tile::Empty,
                        c => Tile::Antenna(c),
                    },
                    vec![(x as i64, y as i64)],
                )
            })
        })
        .fold(HashMap::new(), |mut acc, (tile, coords)| {
            acc.entry(tile).or_insert(vec![]).extend(coords);
            acc
        });
    tile_map.remove(&Tile::Empty);
    tile_map
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid_size = get_grid_size(input);
    let tile_map = get_tile_map(input);

    let result = tile_map
        .values()
        .map(|antenna_coords| get_all_antenna_antinodes(antenna_coords))
        .fold(HashSet::new(), |acc, antinodes| {
            acc.union(&antinodes).cloned().collect()
        })
        .iter()
        .filter(|pos| is_inside_grid(pos, &grid_size))
        .count();

    println!("Result: {}", result);

    Some(result as u64)
}

fn get_all_antenna_antinodes(antennas: &Vec<(i64, i64)>) -> HashSet<(i64, i64)> {
    antennas
        .iter()
        .combinations(2)
        .flat_map(|antenna_pairs| {
            let a = antenna_pairs[0];
            let b = antenna_pairs[1];
            let nodes = get_antenna_pair_antinodes(&a, &b);
            nodes
        })
        .collect()
}

fn is_inside_grid(pos: &(i64, i64), grid_size: &(usize, usize)) -> bool {
    pos.0 >= 0 && pos.0 < grid_size.0 as i64 && pos.1 >= 0 && pos.1 < grid_size.1 as i64
}

fn get_antenna_pair_antinodes(a: &(i64, i64), b: &(i64, i64)) -> HashSet<(i64, i64)> {
    let (dx, dy) = get_antenna_dist(a, b);
    let a_antinode = (a.0 + dx, a.1 + dy);
    let b_antinode = (b.0 - dx, b.1 - dy);
    let mut antinodes = HashSet::new();
    antinodes.insert(a_antinode);
    antinodes.insert(b_antinode);
    antinodes
}

fn get_antenna_dist(a: &(i64, i64), b: &(i64, i64)) -> (i64, i64) {
    (a.0 - b.0, a.1 - b.1)
}
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_two_part_one() {
        let input = "..........\n\
                            ..........\n\
                            ..........\n\
                            ....a.....\n\
                            ........a.\n\
                            .....a....\n\
                            ..........\n\
                            ......A...\n\
                            ..........\n\
                            ..........\n";
        let result = part_one(input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
