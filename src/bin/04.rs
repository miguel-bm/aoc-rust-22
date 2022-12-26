fn parse_input(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let mut first_part = parts.next().unwrap().split("-");
            let mut second_part = parts.next().unwrap().split("-");

            let first_num = first_part.next().unwrap().parse().unwrap();
            let second_num = first_part.next().unwrap().parse().unwrap();
            let third_num = second_part.next().unwrap().parse().unwrap();
            let fourth_num = second_part.next().unwrap().parse().unwrap();

            ((first_num, second_num), (third_num, fourth_num))
        })
        .collect()
}

fn has_assignment_full_overlap(
    (first_start, first_end): &(u32, u32),
    (second_start, second_end): &(u32, u32),
) -> bool {
    (first_start <= second_start && second_end <= first_end)
        || (second_start <= first_start && first_end <= second_end)
}

fn has_assignment_some_overlap(
    (first_start, first_end): &(u32, u32),
    (second_start, second_end): &(u32, u32),
) -> bool {
    (first_start <= second_start && second_start <= first_end)
        || (second_start <= first_start && first_start <= second_end)
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    // count all full overlaps
    let mut overlaps = 0;
    for (first, second) in parsed_input.iter() {
        if has_assignment_full_overlap(first, second) {
            overlaps += 1;
        }
    }
    Some(overlaps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    // count all partial of full overlaps
    let mut overlaps = 0;
    for (first, second) in parsed_input.iter() {
        if has_assignment_some_overlap(first, second) {
            overlaps += 1;
        }
    }
    Some(overlaps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
