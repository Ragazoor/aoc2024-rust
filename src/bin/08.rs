use std::collections::{HashMap, HashSet};

use itertools::{iterate, Itertools};
use num::Integer;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Antenna(char),
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

fn get_antenna_vec(a: &(i64, i64), b: &(i64, i64)) -> (i64, i64) {
    let (dx, dy) = get_antenna_dist(a, b);
    get_unit_vector(dx, dy)
}
fn get_unit_vector(dx: i64, dy: i64) -> (i64, i64) {
    let gcd = dx.gcd(&dy);
    (dx / gcd, dy / gcd)
}

fn get_all_antenna_antinodes_two(
    antennas: &Vec<(i64, i64)>,
    grid_size: &(usize, usize),
) -> HashSet<(i64, i64)> {
    antennas
        .iter()
        .combinations(2)
        .flat_map(|antenna_pairs| {
            let a = antenna_pairs[0];
            let b = antenna_pairs[1];
            let nodes = get_antenna_pair_antinodes_two(&a, &b, grid_size);
            nodes
        })
        .collect()
}

fn get_antenna_pair_antinodes_two(
    a: &(i64, i64),
    b: &(i64, i64),
    grid_size: &(usize, usize),
) -> HashSet<(i64, i64)> {
    let (dx, dy) = get_antenna_vec(a, b);
    let a_antinodes = iterate(0, |n| n + 1)
        .map(|n| (a.0 + n * dx, a.1 + n * dy))
        .take_while(|pos| is_inside_grid(pos, grid_size))
        .collect::<HashSet<(i64, i64)>>();
    let b_antinodes = iterate(0, |n| n + 1)
        .map(|n| (b.0 - n * dx, b.1 - n * dy))
        .take_while(|pos| is_inside_grid(pos, grid_size))
        .collect::<HashSet<(i64, i64)>>();
    println!(
        "antinodes: {:?}",
        a_antinodes.union(&b_antinodes).cloned().collect::<Vec<_>>()
    );
    a_antinodes.union(&b_antinodes).cloned().collect()
}

fn get_antenna_dist(a: &(i64, i64), b: &(i64, i64)) -> (i64, i64) {
    (a.0 - b.0, a.1 - b.1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid_size = get_grid_size(input);
    let tile_map = get_tile_map(input);

    let result = tile_map
        .values()
        .map(|antenna_coords| get_all_antenna_antinodes_two(antenna_coords, &grid_size))
        .fold(HashSet::new(), |acc, antinodes| {
            acc.union(&antinodes).cloned().collect()
        });

    print_grid(input, &result);
    Some(result.iter().count() as u64)
}

fn print_grid(input: &str, result: &HashSet<(i64, i64)>) {
    let grid_size = get_grid_size(input);
    for y in 0..grid_size.1 {
        for x in 0..grid_size.0 {
            let pos = (x as i64, y as i64);
            if result.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
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
        assert_eq!(result, Some(34));
    }
}
