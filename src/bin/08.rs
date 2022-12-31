use advent_of_code::helpers::Matrix2D;

fn parse_input(input: &str) -> Matrix2D<u8> {
    // parse lines of contiguous digits into a vector of vectors of digits
    Matrix2D::from_vec(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    )
}

fn get_direction_visibility_array(tree_heights: &Matrix2D<u8>, direction: i32) -> Matrix2D<bool> {
    let rotated_tree_heights = tree_heights.get_rotation(direction);
    let (rows, cols) = rotated_tree_heights.shape();
    let mut visibility_array = Matrix2D::new(rows, cols, false);

    for (i, row) in rotated_tree_heights.iter_rows().enumerate() {
        let mut current_height = 0 as u8;
        for (j, &value) in row.iter().enumerate() {
            if value > &current_height || j == 0 {
                visibility_array.set(i, j, true);
                current_height = *value;
            }
        }
    }
    visibility_array.get_rotation(-direction)
}

fn get_scenic_score(a: u32, b: u32, c: u32, d: u32) -> u32 {
    a * b * c * d
}

fn get_direction_view_distance_array(tree_heights: &Matrix2D<u8>, direction: i32) -> Matrix2D<u32> {
    let rotated_tree_heights = tree_heights.get_rotation(direction);
    let (rows, cols) = rotated_tree_heights.shape();
    let mut visibility_array = Matrix2D::new(rows, cols, 0 as u32);

    for (i, row) in rotated_tree_heights.iter_rows().enumerate() {
        let mut current_visibility = Vec::new();
        let mut max_height = 0 as u8;
        for (j, &current_height) in row.iter().enumerate() {
            if current_height > &max_height || j == 0 {
                visibility_array.set(i, j, j as u32);
                max_height = *current_height;
                current_visibility = vec![(j, current_height)];
            } else if current_height == &max_height {
                // visibility reaches to previous maximum height tree in current_visibility, which is the first element always
                let visibility_distance = (j - current_visibility[0].0) as u32;
                visibility_array.set(i, j, visibility_distance);
                current_visibility = vec![(j, current_height)];
            } else {
                // value < max_height
                // visibility reaches to first tree in current_visibility with height >= value, including it
                let mut visibility_distance = 0 as u32;
                for (k, &(position, height)) in current_visibility.iter().rev().enumerate() {
                    if height == current_height {
                        visibility_distance = (j - position) as u32;
                        current_visibility =
                            current_visibility[..(current_visibility.len() - k - 1)].to_vec();
                        break;
                    } else if height > current_height {
                        visibility_distance = (j - position) as u32;
                        current_visibility =
                            current_visibility[..(current_visibility.len() - k)].to_vec();
                        break;
                    }
                }
                visibility_array.set(i, j, visibility_distance);
                current_visibility.push((j, current_height));
            }
        }
    }
    visibility_array.get_rotation(-direction)
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree_heights = parse_input(input);
    let north_visibility = get_direction_visibility_array(&tree_heights, 0);
    let east_visibility = get_direction_visibility_array(&tree_heights, 1);
    let south_visibility = get_direction_visibility_array(&tree_heights, 2);
    let west_visibility = get_direction_visibility_array(&tree_heights, 3);

    let mut total_visible = 0 as u32;
    for i in 0..tree_heights.rows {
        for j in 0..tree_heights.cols {
            if *north_visibility.get(i, j)
                || *east_visibility.get(i, j)
                || *south_visibility.get(i, j)
                || *west_visibility.get(i, j)
            {
                total_visible += 1;
            }
        }
    }
    Some(total_visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree_heights = parse_input(input);
    let north_view_distance = get_direction_view_distance_array(&tree_heights, 0);
    let east_view_distance = get_direction_view_distance_array(&tree_heights, 1);
    let south_view_distance = get_direction_view_distance_array(&tree_heights, 2);
    let west_view_distance = get_direction_view_distance_array(&tree_heights, 3);

    let mut max_scenic_score = 0 as u32;
    for i in 0..tree_heights.rows {
        for j in 0..tree_heights.cols {
            let north = *north_view_distance.get(i, j);
            let east = *east_view_distance.get(i, j);
            let south = *south_view_distance.get(i, j);
            let west = *west_view_distance.get(i, j);
            let scenic_score = get_scenic_score(north, east, south, west);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    Some(max_scenic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
