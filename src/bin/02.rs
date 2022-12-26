use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(String, String, String)> {
    let opponent_hands = HashMap::from([("A", "rock"), ("B", "paper"), ("C", "scissors")]);
    let my_hands = HashMap::from([("X", "rock"), ("Y", "paper"), ("Z", "scissors")]);
    let results = HashMap::from([("X", "lose"), ("Y", "draw"), ("Z", "win")]);

    let lines = input.lines();

    let mut line_elements: Vec<&str>;
    let mut parsed_lines = Vec::new();

    for line in lines {
        // get two String elements per line, spearated by a whitespace
        line_elements = line.split_whitespace().collect();
        let opponent_hand = opponent_hands.get(line_elements[0]).unwrap().to_string();
        let my_hand = my_hands.get(line_elements[1]).unwrap().to_string();
        let result = results.get(line_elements[1]).unwrap().to_string();
        // change to String
        parsed_lines.push((opponent_hand, my_hand, result));
    }
    parsed_lines
}

fn rock_paper_scissors_winner(opponent_hand: &str, my_hand: &str) -> String {
    if opponent_hand == my_hand {
        return "draw".to_string();
    }
    let winner = match (opponent_hand, my_hand) {
        ("rock", "paper") => "me",
        ("rock", "scissors") => "opponent",
        ("paper", "rock") => "opponent",
        ("paper", "scissors") => "me",
        ("scissors", "rock") => "me",
        ("scissors", "paper") => "opponent",
        _ => "error",
    };
    winner.to_string()
}

fn rock_paper_scissors_decisor(opponent_hand: &str, desired_result: &str) -> String {
    if desired_result == "draw" {
        return opponent_hand.to_string();
    }
    let winner = match (opponent_hand, desired_result) {
        ("rock", "win") => "paper",
        ("rock", "lose") => "scissors",
        ("paper", "win") => "scissors",
        ("paper", "lose") => "rock",
        ("scissors", "win") => "rock",
        ("scissors", "lose") => "paper",
        _ => "error",
    };
    winner.to_string()
}

fn score_play(opponent_hand: &str, my_hand: &str) -> u32 {
    let winner = rock_paper_scissors_winner(opponent_hand, my_hand);
    match winner.as_str() {
        "me" => 6,
        "opponent" => 0,
        "draw" => 3,
        _ => 0,
    }
}

fn score_hand(hand: &str) -> u32 {
    match hand {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    let mut total_score = 0;
    for (opponent_hand, my_hand, _) in parsed_input {
        total_score += score_play(&opponent_hand, &my_hand);
        total_score += score_hand(&my_hand);
    }
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = parse_input(input);
    let mut total_score = 0;
    for (opponent_hand, _, result) in parsed_input {
        let my_hand = rock_paper_scissors_decisor(&opponent_hand, &result);
        total_score += score_play(&opponent_hand, &my_hand);
        total_score += score_hand(&my_hand);
    }
    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
