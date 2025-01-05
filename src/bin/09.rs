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
    let bytes = parse_input(input);
    let result: usize = get_defragmented_bytes_2(&bytes)
        .iter()
        .enumerate()
        .map(|(i, byte)| match byte {
            Byte::File(idx) => i * idx,
            Byte::Empty => 0,
        })
        .sum();
    Some(result as u64)
}

fn get_defragmented_bytes_2(init_bytes: &Vec<Byte>) -> Vec<Byte> {
    let mut bytes = init_bytes.clone();
    let mut j = bytes.len() - 1;

    while j > 0 {
        //println!("j: {}", j);
        //println!("{:?}", bytes);
        let j_byte = &bytes[j];
        match j_byte {
            Byte::File(_) => {
                let start_idx_file_block = get_start_idx_file_block_2(&bytes, &j);
                //println!("start_idx_file_block: {}", start_idx_file_block);
                let file_block_length = j - &start_idx_file_block + 1;
                //println!("file_block_length: {}", file_block_length);
                let empty_start_idx = get_ok_empty_space_start_idx(&bytes, &file_block_length);
                //println!("empty_start_idx: {:?}", empty_start_idx);
                match empty_start_idx {
                    Some(empty_start_idx) if empty_start_idx < start_idx_file_block => {
                        move_file_block(&mut bytes, empty_start_idx, start_idx_file_block);
                        j = start_idx_file_block;
                    }
                    _ => {
                        j = start_idx_file_block - 1;
                    }
                }
            }
            Byte::Empty => {
                j -= 1;
            }
        }
    }
    //println!("{:?}", bytes);
    bytes
}

fn move_file_block(bytes: &mut Vec<Byte>, mut i: usize, mut j: usize) {
    let file_id = match bytes[j] {
        Byte::File(id) => id,
        _ => panic!("Invalid byte"),
    };

    let len_bytes = bytes.len();

    while j < len_bytes {
        let j_byte = bytes[j];
        match j_byte {
            Byte::File(id) => {
                if id != file_id {
                    break;
                } else {
                    bytes[j] = Byte::Empty;
                    bytes[i] = j_byte;
                    i += 1;
                    j += 1;
                }
            }
            Byte::Empty => {
                break;
            }
        }
    }
}

fn get_empty_space_length(bytes: &Vec<Byte>, i: &usize) -> usize {
    let mut j = *i;
    let mut j_byte = bytes[j];
    let len = bytes.len();
    while j_byte == Byte::Empty && j < (len - 1) {
        j += 1;
        j_byte = bytes[j];
    }
    j - i
}

fn get_ok_empty_space_start_idx(bytes: &Vec<Byte>, min_length: &usize) -> Option<usize> {
    let mut i = 0;
    let len = bytes.len();
    while i < (len - 1) {
        let i_byte = bytes[i];
        match i_byte {
            Byte::Empty => {
                let empty_space_len = get_empty_space_length(bytes, &i);
                if empty_space_len >= *min_length {
                    return Some(i);
                } else {
                    i += empty_space_len;
                }
            }
            Byte::File(_) => {
                i += 1;
            }
        }
    }
    None
}

fn get_start_idx_file_block_2(bytes: &Vec<Byte>, i: &usize) -> usize {
    let mut j = *i;
    let mut current_byte = bytes[j];
    let init_byte = bytes[j];
    while current_byte == init_byte && j > 0 {
        j -= 1;
        current_byte = bytes[j];
    }
    j + 1
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
        assert_eq!(result, Some(2858));
    }
    #[test]
    fn test_two_part_two() {
        // parse to "0..111.22.333"
        let result = part_two("1231213");
        assert_eq!(result, Some(117));
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
