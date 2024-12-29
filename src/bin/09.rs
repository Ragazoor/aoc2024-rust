advent_of_code::solution!(9);

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Byte {
    Empty,
    File(usize),
}

fn parse_input(input: &str) -> Vec<Byte> {
    let mut idx = 0;
    input
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let num = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                let bytes = parse_file(idx, num);
                idx += 1;
                bytes
            } else {
                parse_empty_space(num)
            }
        })
        .collect()
}

fn parse_file(idx: usize, num: u32) -> Vec<Byte> {
    vec![Byte::File(idx); num as usize]
}

fn parse_empty_space(num: u32) -> Vec<Byte> {
    vec![Byte::Empty; num as usize]
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut bytes = parse_input(input);
    move_bytes(&mut bytes);
    let result: usize = bytes
        .iter()
        .filter(|byte| **byte != Byte::Empty)
        .enumerate()
        .map(|(i, byte)| match byte {
            Byte::File(idx) => i * idx,
            Byte::Empty => 0,
        })
        .sum();
    Some(result as u64)
}

fn move_bytes(bytes: &mut Vec<Byte>) {
    let mut i = 0;
    let mut j = bytes.len() - 1;
    while i <= j {
        let i_byte = &bytes[i];
        match i_byte {
            Byte::File(_) => {
                i += 1;
            }
            Byte::Empty => {
                j = get_next_j(&bytes, j);
                if j <= i {
                    break;
                }
                let j_byte = bytes[j];
                bytes[j] = Byte::Empty;
                bytes[i] = j_byte;
                i += 1;
            }
        }
    }
}

fn get_next_j(bytes: &Vec<Byte>, mut j: usize) -> usize {
    let mut j_byte = bytes[j];
    while j_byte == Byte::Empty {
        j -= 1;
        j_byte = bytes[j];
    }
    j
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }
    #[test]
    fn test_two_part_one() {
        let result = part_one("1111111111");
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse() {
        let result = parse_input("12345");
        let expected = vec![
            Byte::File(0),
            Byte::Empty,
            Byte::Empty,
            Byte::File(1),
            Byte::File(1),
            Byte::File(1),
            Byte::Empty,
            Byte::Empty,
            Byte::Empty,
            Byte::Empty,
            Byte::File(2),
            Byte::File(2),
            Byte::File(2),
            Byte::File(2),
            Byte::File(2),
        ];
        assert_eq!(result, expected);
    }
}
