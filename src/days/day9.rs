use std::collections::{HashSet, VecDeque};

struct Day9 {}

impl Day9 {
    pub fn smoke_basin(inputs: Vec<String>) -> i32 {
        if inputs.is_empty() {
            return 0;
        }

        let mut map: Vec<Vec<i32>> = Vec::new();

        for input in inputs {
            map.push(input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect());
        }

        let (n, m) = (map.len(), map[0].len());

        fn is_low_point(i: usize, j: usize, n: usize, m: usize, map: &Vec<Vec<i32>>) -> bool {
            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            for dir in dirs {
                let (x, y) = (i as i32 + dir.0, j as i32 + dir.1);

                if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 {
                    continue;
                }

                if map[x as usize][y as usize] <= map[i][j] {
                    return false;
                }
            }

            true
        }

        let mut risk_level = 0;
        for i in 0..n {
            for j in 0..m {
                if is_low_point(i, j, n, m, &map) {
                    risk_level += map[i][j] + 1;
                }
            }
        }

        risk_level
    }

    pub fn largest_basin(inputs: Vec<String>) -> i32 {
        if inputs.is_empty() {
            return 0;
        }

        let mut map: Vec<Vec<i32>> = Vec::new();

        for input in inputs {
            map.push(input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect());
        }

        let (n, m) = (map.len(), map[0].len());

        fn is_low_point(i: usize, j: usize, n: usize, m: usize, map: &Vec<Vec<i32>>) -> bool {
            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            for dir in dirs {
                let (x, y) = (i as i32 + dir.0, j as i32 + dir.1);

                if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 {
                    continue;
                }

                if map[x as usize][y as usize] <= map[i][j] {
                    return false;
                }
            }

            true
        }

        fn bfs_basin_size(i: usize, j: usize, n: usize, m: usize, map: &Vec<Vec<i32>>) -> i32 {
            let mut basin_size = 1;
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            let mut seen: HashSet<(usize, usize)> = HashSet::new();

            queue.push_back((i, j));
            seen.insert((i, j));

            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            while !queue.is_empty() {
                let (i, j) = queue.pop_front().unwrap();

                for dir in dirs {
                    let (x, y) = (i as i32 + dir.0, j as i32 + dir.1);

                    if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 {
                        continue;
                    }

                    if seen.contains(&(x as usize, y as usize)) {
                        continue;
                    }
                    seen.insert((x as usize, y as usize));

                    if map[x as usize][y as usize] < 9 {
                        basin_size += 1;
                        queue.push_back((x as usize, y as usize));
                    }
                }
            }

            basin_size
        }

        let mut risk_levels: Vec<i32> = Vec::new();
        for i in 0..n {
            for j in 0..m {
                if is_low_point(i, j, n, m, &map) {
                    risk_levels.push(bfs_basin_size(i, j, n, m, &map));
                }
            }
        }

        risk_levels.sort_unstable();
        let n = risk_levels.len();
        risk_levels[n - 1] * risk_levels[n - 2] * risk_levels[n - 3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_smoke_basin_example() {
        let input = utils::read_file_lines("input/day9_1.txt".to_string());
        assert_eq!(Day9::smoke_basin(input), 15);
    }

    #[test]
    fn test_smoke_basin_empty() {
        assert_eq!(Day9::smoke_basin(Vec::new()), 0);
    }

    #[test]
    fn test_smoke_basin() {
        let input = utils::read_file_lines("input/day9_2.txt".to_string());
        assert_eq!(Day9::smoke_basin(input), 554);
    }

    #[test]
    fn test_largest_basin_example() {
        let input = utils::read_file_lines("input/day9_1.txt".to_string());
        assert_eq!(Day9::largest_basin(input), 1134);
    }

    #[test]
    fn test_largest_basin_empty() {
        assert_eq!(Day9::largest_basin(Vec::new()), 0);
    }

    #[test]
    fn test_largest_basin() {
        let input = utils::read_file_lines("input/day9_2.txt".to_string());
        assert_eq!(Day9::largest_basin(input), 1017792);
    }
}
