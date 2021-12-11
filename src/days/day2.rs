struct Day2 {}

impl Day2 {
    pub fn dive(input: Vec<String>) -> i32 {
        let sonars: Vec<(String, i32)> = input
            .into_iter()
            .map(|s| {
                let parts = s.split(' ').collect::<Vec<&str>>();
                (parts[0].to_string(), parts[1].parse::<i32>().unwrap())
            })
            .collect();

        let mut pos = (0, 0);

        for sonar in sonars {
            let (dir, dist) = sonar;
            match dir.as_str() {
                "up" => pos.1 -= dist,
                "down" => pos.1 += dist,
                "forward" => pos.0 += dist,
                _ => panic!("Unknown direction: {}", dir),
            }
        }

        pos.0 * pos.1
    }

    pub fn dive_aim(input: Vec<String>) -> i32 {
        let sonars: Vec<(String, i32)> = input
            .into_iter()
            .map(|s| {
                let parts = s.split(' ').collect::<Vec<&str>>();
                (parts[0].to_string(), parts[1].parse::<i32>().unwrap())
            })
            .collect();

        let mut aim = 0;
        let mut pos = (0, 0);

        for sonar in sonars {
            let (dir, dist) = sonar;
            match dir.as_str() {
                "up" => aim -= dist,
                "down" => aim += dist,
                "forward" => {
                    pos.0 += dist;
                    pos.1 += aim * dist;
                },
                _ => panic!("Unknown direction: {}", dir),
            }
        }

        pos.0 * pos.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_dive_example() {
        let input = utils::read_file_lines("input/day2_1.txt".to_string());
        assert_eq!(Day2::dive(input), 150);
    }

    #[test]
    fn test_dive_empty() {
        assert_eq!(Day2::dive(Vec::new()), 0);
    }

    #[test]
    fn test_dive() {
        let input = utils::read_file_lines("input/day2_2.txt".to_string());
        assert_eq!(Day2::dive(input), 1648020);
    }

    #[test]
    fn test_dive_aim_example() {
        let input = utils::read_file_lines("input/day2_1.txt".to_string());
        assert_eq!(Day2::dive_aim(input), 900);
    }

    #[test]
    fn test_dive_aim_empty() {
        assert_eq!(Day2::dive_aim(Vec::new()), 0);
    }

    #[test]
    fn test_dive_aim() {
        let input = utils::read_file_lines("input/day2_2.txt".to_string());
        assert_eq!(Day2::dive_aim(input), 1759818555);
    }
}
