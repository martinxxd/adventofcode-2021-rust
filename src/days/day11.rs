use std::collections::{HashSet, VecDeque};

struct Day11 {}

impl Day11 {
    pub fn dumbo_octopus(lines: Vec<String>, simultaneously: bool) -> i32 {
        let mut grid = vec![vec![0; 10]; 10];

        let mut i = 0;
        for line in lines.iter() {
            for (j, ch) in line.chars().enumerate() {
                grid[i][j] = ch.to_digit(10).unwrap();
            }
            i += 1;
        }

        let mut num_flashes = 0;

        let mut steps = 0;
        let mut max_steps = 100;
        if simultaneously {
            max_steps = 10000;
        }

        fn chaining(grid: &mut Vec<Vec<u32>>, pos: Vec<(usize, usize)>, num_flashes: &mut i32) -> usize {
            let mut queue = VecDeque::new();
            let mut seen = HashSet::new();

            let dirs: [(i32, i32); 8] = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

            for p in pos {
                grid[p.0][p.1] = 0;
                *num_flashes += 1;

                queue.push_back(p);
                seen.insert((p.0 as usize, p.1 as usize));
            }

            while queue.len() > 0 {
                let (i, j) = queue.pop_front().unwrap();

                for dir in dirs {
                    let (mut i, mut j) = (i as i32, j as i32);

                    i += dir.0;
                    j += dir.1;

                    if i < 0 || j < 0 || i >= 10 || j >= 10 {
                        continue;
                    }

                    let (i, j) = (i as usize, j as usize);

                    if seen.contains(&(i, j)) {
                        continue;
                    }

                    grid[i][j] = u32::min(grid[i][j] + 1, 10);

                    if grid[i][j] > 9 {
                        grid[i][j] = 0;
                        *num_flashes += 1;
                        seen.insert((i, j));
                        queue.push_back((i, j));
                    }
                }
            }

            seen.len()
        }

        while steps < max_steps {
            let mut flashed = Vec::<(usize, usize)>::new();
            for i in 0..10 {
                for j in 0..10 {
                    grid[i][j] = u32::min(grid[i][j] + 1, 10);
                    if grid[i][j] > 9 {
                        flashed.push((i, j));
                    }
                }
            }

            let flashed_step = chaining(&mut grid, flashed, &mut num_flashes);
            if simultaneously && flashed_step == 100 {
                return steps + 1;
            }
            steps += 1;
        }

        if simultaneously {
            return -1;
        } else {
            num_flashes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_dumbo_octopus_example() {
        let input = utils::read_file_lines("input/day11_1.txt".to_string());
        assert_eq!(Day11::dumbo_octopus(input, false), 1656);
    }

    #[test]
    fn test_dumbo_octopus() {
        let input = utils::read_file_lines("input/day11_2.txt".to_string());
        assert_eq!(Day11::dumbo_octopus(input, false), 1644);
    }

    #[test]
    fn test_dumbo_octopus_all_example() {
        let input = utils::read_file_lines("input/day11_1.txt".to_string());
        assert_eq!(Day11::dumbo_octopus(input, true), 195);
    }

    #[test]
    fn test_dumbo_octopus_all() {
        let input = utils::read_file_lines("input/day11_2.txt".to_string());
        assert_eq!(Day11::dumbo_octopus(input, true), 229);
    }
}
