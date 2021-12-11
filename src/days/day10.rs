use std::collections::VecDeque;

struct Day10 {}

impl Day10 {
    pub fn syntax_scoring(lines: Vec<String>) -> i64 {
        let mut points = 0;
        for line in lines.iter() {
            let ch = Day10::parse_line(line).0;
            if ch.is_some() {
                points += Day10::char_to_score(ch.unwrap());
            }
        }
        points
    }

    pub fn syntax_scoring_incomplete(lines: Vec<String>) -> i64 {
        let mut all_points: Vec<i64> = vec![];
        for line in lines.iter() {
            let mut points = 0;
            let (ch, queue) = Day10::parse_line(line);
            if ch.is_none() && queue.is_some() {
                let mut queue = queue.unwrap();
                while queue.len() > 0 {
                    let ch = queue.pop_back().unwrap();
                    points = (points * 5) + Day10::char_to_score_incomplete(Day10::char_opening_to_closing(ch));
                }

                all_points.push(points);
            }
        }

        all_points.sort();
        if !all_points.is_empty() {
            all_points[all_points.len() / 2]
        } else {
            0
        }
    }

    fn parse_line(line: &String) -> (Option<char>, Option<VecDeque<char>>) {
        let mut queue = VecDeque::<char>::new();
        for ch in line.chars() {
            if Day10::char_is_closing(ch) {
                if queue.back() == Some(&Day10::char_closing_to_opening(ch)) {
                    queue.pop_back();
                } else {
                    return (Some(ch), None);
                }
            } else {
                queue.push_back(ch);
            }
        }

        (None, Some(queue))
    }

    fn char_to_score(ch: char) -> i64 {
        match ch {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }

    fn char_to_score_incomplete(ch: char) -> i64 {
        match ch {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    }

    fn char_closing_to_opening(ch: char) -> char {
        match ch {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            '>' => '<',
            _ => ' ',
        }
    }

    fn char_opening_to_closing(ch: char) -> char {
        match ch {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => ' ',
        }
    }

    fn char_is_closing(ch: char) -> bool {
        match ch {
            ')' => true,
            ']' => true,
            '}' => true,
            '>' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_syntax_scoring_example() {
        let input = utils::read_file_lines("input/day10_1.txt".to_string());
        assert_eq!(Day10::syntax_scoring(input), 26397);
    }

    #[test]
    fn test_syntax_scoring_empty() {
        assert_eq!(Day10::syntax_scoring(Vec::new()), 0);
    }

    #[test]
    fn test_syntax_scoring() {
        let input = utils::read_file_lines("input/day10_2.txt".to_string());
        assert_eq!(Day10::syntax_scoring(input), 415953);
    }

    #[test]
    fn test_syntax_scoring_all_example() {
        let input = utils::read_file_lines("input/day10_1.txt".to_string());
        assert_eq!(Day10::syntax_scoring_incomplete(input), 288957);
    }

    #[test]
    fn test_syntax_scoring_all_empty() {
        assert_eq!(Day10::syntax_scoring_incomplete(Vec::new()), 0);
    }

    #[test]
    fn test_syntax_scoring_all() {
        let input = utils::read_file_lines("input/day10_2.txt".to_string());
        assert_eq!(Day10::syntax_scoring_incomplete(input), 2292863731);
    }
}
