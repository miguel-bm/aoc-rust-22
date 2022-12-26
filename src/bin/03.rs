fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let length = line.len();
            let half_length = length / 2;
            let first_compartment = &line[0..half_length];
            let second_compartment = &line[half_length..length];
            (first_compartment, second_compartment)
        })
        .collect()
}

fn get_priority_score(item: &char) -> u32 {
    let priority_order = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let priority_score = (priority_order.find(*item).unwrap() + 1) as u32;
    priority_score
}

fn find_same_char(first: &str, second: &str) -> Option<char> {
    for char in first.chars() {
        // check if char in second
        if second.contains(char) {
            return Some(char);
        }
    }
    None
}

fn find_same_char_three(first: &str, second: &str, third: &str) -> Option<char> {
    for char in first.chars() {
        // check if char in second
        if second.contains(char) && third.contains(char) {
            return Some(char);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    let mut total_priority_score = 0;
    for (first, second) in parsed_input {
        let same_char = find_same_char(first, second);
        if let Some(char) = same_char {
            let sack_score = get_priority_score(&char);
            total_priority_score += sack_score;
        } else {
            panic!("No match found between {} and {}", first, second);
        }
    }
    Some(total_priority_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    // make sure that the amount of rucksacks is a multiple of 3
    assert!(parsed_input.len() % 3 == 0);
    // join the rucksack compartments from (String, String) to concatenated String
    let parsed_input = parsed_input
        .iter()
        .map(|(first, second)| [first.to_string(), second.to_string()].join(""))
        .collect::<Vec<String>>();

    // divide the rucksacks in groups of 3
    let elf_team_rucksacks = parsed_input.chunks(3);
    let mut total_priority_score = 0;
    for elf_team_rucksack in elf_team_rucksacks {
        let same_char = find_same_char_three(
            elf_team_rucksack[0].as_str(),
            elf_team_rucksack[1].as_str(),
            elf_team_rucksack[2].as_str(),
        );
        if let Some(char) = same_char {
            let sack_score = get_priority_score(&char);
            total_priority_score += sack_score;
        } else {
            panic!(
                "No match found between {}. {} and {}",
                elf_team_rucksack[0], elf_team_rucksack[1], elf_team_rucksack[2]
            );
        }
    }
    Some(total_priority_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
