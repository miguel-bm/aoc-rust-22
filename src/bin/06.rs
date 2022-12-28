fn parse_input(input: &str) -> Vec<char> {
    input.chars().filter(|c| c.is_alphabetic()).collect()
}

fn locate_unique_sequence(input: &Vec<char>, sequence_length: &usize) -> Option<u32> {
    let length = input.len();
    for i in (sequence_length - 1)..length {
        let chunk = input[i - (sequence_length - 1)..i + 1].to_vec();
        // are all characters in the chunk unique?
        if chunk
            .iter()
            .all(|c| chunk.iter().filter(|&x| x == c).count() == 1)
        {
            return Some(i as u32 + 1);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    let sequence_length = 4 as usize;
    locate_unique_sequence(&parsed_input, &sequence_length)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    let sequence_length = 14 as usize;
    locate_unique_sequence(&parsed_input, &sequence_length)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
