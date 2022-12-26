fn parse_input(input: &str) -> Vec<Vec<u32>> {
    // parse a vector of vectors of u32 from a string. Each line is a u32 or a newline
    // 1 newline is a separator of u32, 2 newlines is a separator of vectors

    // Split input into lines
    let lines = input.lines();

    // Parse lines into u32
    let mut parsed_group = Vec::new();
    let mut parsed_lines = Vec::new();
    for line in lines {
        if line.is_empty() {
            parsed_lines.push(parsed_group);
            parsed_group = Vec::new();
        } else {
            parsed_group.push(line.parse().unwrap());
        }
    }
    parsed_lines.push(parsed_group);
    parsed_lines
}

fn get_calories_per_elf(input: Vec<Vec<u32>>) -> Vec<u32> {
    let elf_calories: Vec<u32> = input.iter().map(|group| group.iter().sum()).collect();
    elf_calories
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    let mut elf_calories = get_calories_per_elf(parsed_input);
    elf_calories.sort();
    elf_calories.reverse();

    elf_calories.first().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    let mut elf_calories = get_calories_per_elf(parsed_input);
    elf_calories.sort();
    elf_calories.reverse();

    let first_three_sum = elf_calories.iter().take(3).sum::<u32>();
    Some(first_three_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
