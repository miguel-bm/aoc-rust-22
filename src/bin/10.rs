use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            if line == "noop" {
                return (1, 0);
            } else {
                let split = line.split(" ");
                let val = split.last().unwrap().parse::<i32>().unwrap();
                return (2, val);
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = parse_input(input);
    let mut interesting_cycles = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut interesting_cycles_signal_strengths = Vec::new();
    let mut state = (1, 1);
    for (cycles_to_completion, add_quantity) in instructions.iter() {
        for i in 0..cycles_to_completion.clone() {
            let cycle = state.0 + i;
            if interesting_cycles.contains(&cycle) {
                let signal_strength = state.1 * cycle;
                interesting_cycles_signal_strengths.push(signal_strength);
                interesting_cycles.remove(&(cycle));
            }
        }
        state = (state.0 + cycles_to_completion, state.1 + add_quantity);
    }
    // gather the second elements of instructions in a vector
    Some(interesting_cycles_signal_strengths.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let mut register = 1;
    let mut cycle = 0;
    let mut pixels = Vec::new();
    for (cycles_to_completion, add_quantity) in instructions.iter() {
        for _ in 0..cycles_to_completion.clone() {
            let column = cycle % 40;
            pixels.push(column >= register - 1 && column <= register + 1);
            cycle += 1;
        }
        register += add_quantity;
    }
    // get pixels in chunks of 40
    for row in pixels.chunks(40) {
        for pixel in row {
            if *pixel {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
