use std::collections::HashMap;

use nom::{
    branch::Permutation, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    multi::separated_list1, IResult,
};

use itertools::{repeat_n, Itertools};

advent_of_code::solution!(7);

fn parse_input(input: &str) -> IResult<&str, HashMap<i64, Vec<i64>>> {
    let (input, lines) = separated_list1(tag("\n"), parse_line)(input)?;
    Ok((input, lines.into_iter().collect()))
}

fn parse_line(input: &str) -> IResult<&str, (i64, Vec<i64>)> {
    let (input, key) = parse_number(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, num_vec) = separated_list1(tag(" "), parse_number)(input)?;
    Ok((input, (key, num_vec)))
}

fn parse_number(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Multiply,
    Concatenate,
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, equations) = parse_input(input).ok()?;
    let sum = solution1(equations);
    Some(sum)
}

const PART1_OPERATIONS: [Operator; 2] = [Operator::Plus, Operator::Multiply];
const PART2_OPERATIONS: [Operator; 3] = [Operator::Plus, Operator::Multiply, Operator::Concatenate];

fn solution1(equations: HashMap<i64, Vec<i64>>) -> u64 {
    equations.iter().fold(0, |acc, (key, value)| {
        if is_solveable_equation(key, value, &PART1_OPERATIONS.to_vec()) {
            acc + (*key as u64)
        } else {
            acc
        }
    })
}

fn is_solveable_equation(key: &i64, equation: &Vec<i64>, operators: &Vec<Operator>) -> bool {
    get_all_permutations(equation.len() - 1, operators)
        .iter()
        .any(|operations| calculate_equation(equation, operations) == *key)
}

fn get_all_permutations(num_operators: usize, operators: &Vec<Operator>) -> Vec<Vec<Operator>> {
    repeat_n(operators.clone(), num_operators)
        .multi_cartesian_product()
        .collect()
}

fn calculate_equation(equation: &Vec<i64>, operations: &Vec<Operator>) -> i64 {
    if equation.len() == 1 {
        return equation[0];
    } else {
        let head = equation[0];
        let tail = &equation[1..];
        tail.iter()
            .enumerate()
            .fold(head, |acc, (i, val)| calculate(&operations[i], &acc, val))
    }
}

fn calculate(operation: &Operator, a: &i64, b: &i64) -> i64 {
    match operation {
        Operator::Plus => a + b,
        Operator::Multiply => a * b,
        Operator::Concatenate => {
            let a_str = a.to_string();
            let b_str = b.to_string();
            format!("{}{}", a_str, b_str).parse::<i64>().unwrap()
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, equations) = parse_input(input).ok()?;
    let sum = solution2(equations);

    Some(sum)
}

fn solution2(equations: HashMap<i64, Vec<i64>>) -> u64 {
    equations.iter().fold(0, |acc, (key, value)| {
        if is_solveable_equation(key, value, &PART2_OPERATIONS.to_vec()) {
            acc + (*key as u64)
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
