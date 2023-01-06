use ::std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let direction_str = split.next().unwrap();
            let direction = match direction_str {
                "R" => 0,
                "U" => 1,
                "L" => 2,
                "D" => 3,
                _ => panic!("Invalid direction: {}", direction_str),
            };
            let steps = split.next().unwrap().parse::<u8>().unwrap();
            (direction, steps)
        })
        .collect()
}

fn move_knot(knot_position: (i32, i32), header_knot_position: (i32, i32)) -> (i32, i32) {
    let (x, y) = knot_position;
    let (x_header, y_header) = header_knot_position;
    let does_move = (x - x_header).abs() > 1 || (y - y_header).abs() > 1;
    if does_move {
        // crop diffs to (-1, 1)
        let diff_x = if x_header - x > 0 {
            1
        } else if x_header - x < 0 {
            -1
        } else {
            0
        };
        let diff_y = if y_header - y > 0 {
            1
        } else if y_header - y < 0 {
            -1
        } else {
            0
        };
        return (x + diff_x, y + diff_y);
    } else {
        return (x, y);
    }
}

fn move_rope(knot_positions: Vec<(i32, i32)>, movement: u8) -> Vec<(i32, i32)> {
    let (head_x, head_y) = knot_positions[0];
    let (head_x_new, head_y_new) = match movement {
        0 => (head_x + 1, head_y),
        1 => (head_x, head_y + 1),
        2 => (head_x - 1, head_y),
        3 => (head_x, head_y - 1),
        _ => panic!("Invalid movement: {}", movement),
    };
    let mut new_knot_positions = vec![(head_x_new, head_y_new)];
    for knot_position in knot_positions[1..].iter() {
        let header_knot_position = new_knot_positions.last().unwrap().clone();
        let new_knot_position = move_knot(knot_position.clone(), header_knot_position);
        new_knot_positions.push(new_knot_position);
    }
    new_knot_positions
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    let mut visited_positions = HashSet::new();
    let mut knot_positions = vec![(0, 0), (0, 0)];

    for (movement, steps) in parsed_input {
        for _ in 0..steps {
            knot_positions = move_rope(knot_positions, movement);
            visited_positions.insert(knot_positions.last().unwrap().clone());
        }
    }
    visited_positions.len().try_into().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    let mut visited_positions = HashSet::new();
    let mut knot_positions = vec![(0, 0); 10];

    for (movement, steps) in parsed_input {
        for _ in 0..steps {
            knot_positions = move_rope(knot_positions, movement);
            visited_positions.insert(knot_positions.last().unwrap().clone());
        }
    }
    visited_positions.len().try_into().ok()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
