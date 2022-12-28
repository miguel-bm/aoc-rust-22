fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(u32, u32, u32)>) {
    let mut sections = input.split("\n\n");

    let initial_configurations = sections.next().unwrap().lines();
    let last_config_line = initial_configurations.clone().last().unwrap();
    let num_stacks = last_config_line.split("  ").count() as u32;
    // iterate initial_configurations from the second to last line backwards
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }
    for level in initial_configurations.rev().skip(1) {
        let len_level = level.len();
        for (i, stack) in stacks.iter_mut().enumerate() {
            if len_level > i * 4 + 1 {
                let stack_char = level[i * 4 + 1..i * 4 + 2].chars().next().unwrap();
                if stack_char != ' ' {
                    stack.push(stack_char);
                }
            }
        }
    }

    let instructions = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            // parse move X from Y to Z as three integers
            let mut parts = line.split(" ");
            let move_amount = parts.nth(1).unwrap().parse().unwrap();
            let from_col = parts.nth(1).unwrap().parse().unwrap();
            let to_col = parts.nth(1).unwrap().parse().unwrap();

            (move_amount, from_col, to_col)
        })
        .collect();

    (stacks, instructions)
}

fn execute_instructions_one(
    stacks: &Vec<Vec<char>>,
    instructions: Vec<(u32, u32, u32)>,
) -> Vec<Vec<char>> {
    let mut stacks = stacks.clone();
    for (move_amount, from_col, to_col) in instructions {
        let from_col = from_col as usize - 1;
        let to_col = to_col as usize - 1;

        let from_stack = &mut stacks[from_col];
        let mut to_move = Vec::new();

        for _ in 0..move_amount {
            to_move.push(from_stack.pop().unwrap());
        }
        let to_stack = &mut stacks[to_col];
        for c in to_move.iter() {
            to_stack.push(*c);
        }
    }
    stacks
}

fn execute_instructions_two(
    stacks: &Vec<Vec<char>>,
    instructions: Vec<(u32, u32, u32)>,
) -> Vec<Vec<char>> {
    let mut stacks = stacks.clone();
    for (move_amount, from_col, to_col) in instructions {
        let from_col = from_col as usize - 1;
        let to_col = to_col as usize - 1;

        let from_stack = &mut stacks[from_col];
        let mut to_move = Vec::new();

        for _ in 0..move_amount {
            to_move.push(from_stack.pop().unwrap());
        }
        let to_stack = &mut stacks[to_col];
        for c in to_move.iter().rev() {
            to_stack.push(*c);
        }
    }
    stacks
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks, instructions) = parse_input(input);
    let final_stacks = execute_instructions_one(&stacks, instructions);
    let mut top_boxes = Vec::new();
    for stack in &final_stacks {
        top_boxes.push(stack.last().unwrap());
    }
    let mut result = String::new();
    for c in top_boxes {
        result.push(*c);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks, instructions) = parse_input(input);
    let final_stacks = execute_instructions_two(&stacks, instructions);
    let mut top_boxes = Vec::new();
    for stack in &final_stacks {
        top_boxes.push(stack.last().unwrap());
    }
    let mut result = String::new();
    for c in top_boxes {
        result.push(*c);
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
