use std::collections::HashSet;

struct Day4 {}

impl Day4 {
    pub fn giant_squid(lines: Vec<String>) -> i32 {
        Day4::giant_squid_engine(lines, true)
    }

    pub fn giant_squid_last(lines: Vec<String>) -> i32 {
        Day4::giant_squid_engine(lines, false)
    }

    fn giant_squid_engine(lines: Vec<String>, first: bool) -> i32 {
        let mut rnds = Vec::<i32>::new();

        let mut boards = Vec::<Vec<Vec<i32>>>::new();
        let mut board_idx = 0;
        let mut row_idx = 0;
        boards.push(Vec::<Vec<i32>>::new());

        // build board
        for i in 0..lines.len() {
            let mut line = lines[i].clone();
            line = line.replace("  ", " ");

            if i == 0 {
                rnds = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            } else if i > 1 && !line.is_empty() {
                if line.starts_with(' ') {
                    line = line.chars().skip(1).collect();
                }
                let row: Vec<i32> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
                boards[board_idx].push(row);
                row_idx += 1;
                if row_idx >= 5 {
                    board_idx += 1;
                    row_idx = 0;
                    boards.push(Vec::<Vec<i32>>::new());
                }
            }
        }

        // remove last empty board
        boards.pop();

        fn is_winning_board(board: &Vec<Vec<i32>>) -> bool {
            for i in 0..5_usize {
                let (mut cnt_row, mut cnt_col) = (0, 0);

                for j in 0..5_usize {
                    if board[i][j] == -1 {
                        cnt_row += 1;
                    }
                    if board[j][i] == -1 {
                        cnt_col += 1;
                    }
                }

                if cnt_row == 5 || cnt_col == 5 {
                    return true;
                }
            }

            false
        }

        fn sum_board(board: &Vec<Vec<i32>>) -> i32 {
            let mut sum = 0;
            for i in 0..5_usize {
                for j in 0..5_usize {
                    if board[i][j] != -1 {
                        sum += board[i][j];
                    }
                }
            }

            sum
        }

        let mut won_boards = HashSet::new();
        let mut last_to_win_sum = -1;
        for rnd in rnds {
            for b in 0..boards.len() {
                for i in 0..5_usize {
                    for j in 0..5_usize {
                        if boards[b][i][j] == rnd {
                            boards[b][i][j] = -1;

                            if is_winning_board(&boards[b]) {
                                if first {
                                    return sum_board(&boards[b]) * rnd;
                                } else {
                                    if !won_boards.contains(&b) {
                                        last_to_win_sum = sum_board(&boards[b]) * rnd;
                                    }
                                    won_boards.insert(b);
                                }
                            }
                        }
                    }
                }
            }
        }

        last_to_win_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_giant_squid_example() {
        let input = utils::read_file("input/day4_1.txt".to_string());
        assert_eq!(Day4::giant_squid(input), 4512);
    }

    #[test]
    fn test_giant_squid_empty() {
        assert_eq!(Day4::giant_squid(Vec::new()), -1);
    }

    #[test]
    fn test_giant_squid() {
        let input = utils::read_file("input/day4_2.txt".to_string());
        assert_eq!(Day4::giant_squid(input), 16674);
    }

    #[test]
    fn test_giant_squid_last_example() {
        let input = utils::read_file("input/day4_1.txt".to_string());
        assert_eq!(Day4::giant_squid_last(input), 1924);
    }

    #[test]
    fn test_giant_squid_last_empty() {
        assert_eq!(Day4::giant_squid_last(Vec::new()), -1);
    }

    #[test]
    fn test_giant_squid_last() {
        let input = utils::read_file("input/day4_2.txt".to_string());
        assert_eq!(Day4::giant_squid_last(input), 7075);
    }
}
