struct Day5 {}

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

#[derive(Debug)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        let start = Coord::new(x1, y1);
        let end = Coord::new(x2, y2);
        Line { start, end }
    }

    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn get_direction(&self) -> f32 {
        let x = self.end.x - self.start.x;
        let y = self.end.y - self.start.y;

        (y as f32 / x as f32).atan().to_degrees()
    }
}

impl Day5 {
    pub fn hydrothermal_venture(input: Vec<String>, only_horizontal: bool) -> i32 {
        let mut lines = Vec::<Line>::new();

        let (mut max_x, mut max_y) = (0, 0);

        for line in input {
            let raw_coords = line.split_once(" -> ").unwrap();
            let split_start = raw_coords.0.split_once(",").unwrap();
            let (x1, y1) = (split_start.0.parse::<i32>().unwrap(), split_start.1.parse::<i32>().unwrap());

            let split_end = raw_coords.1.split_once(",").unwrap();
            let (x2, y2) = (split_end.0.parse::<i32>().unwrap(), split_end.1.parse::<i32>().unwrap());

            max_x = i32::max(max_x, x1);
            max_x = i32::max(max_x, x2);
            max_y = i32::max(max_y, y1);
            max_y = i32::max(max_y, y2);

            lines.push(Line::new(x1, y1, x2, y2));
        }

        let mut map = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];

        for line in lines {
            if only_horizontal && !line.is_straight() {
                continue;
            }

            let mut pos = (line.start.x, line.start.y);
            let mut inc = (0, 0);

            if line.start.x > line.end.x {
                inc.0 = -1;
            } else if line.start.x < line.end.x {
                inc.0 = 1;
            }

            if line.start.y > line.end.y {
                inc.1 = -1;
            } else if line.start.y < line.end.y {
                inc.1 = 1;
            }

            while pos.0 != line.end.x + inc.0 || pos.1 != line.end.y + inc.1 {
                map[pos.1 as usize][pos.0 as usize] += 1;
                pos.0 += inc.0;
                pos.1 += inc.1;
            }
        }

        let mut count = 0;
        for i in 0..=max_x {
            for j in 0..=max_y {
                if map[j as usize][i as usize] >= 2 {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_hydrothermal_venture_example() {
        let input = utils::read_file("input/day5_1.txt".to_string());
        assert_eq!(Day5::hydrothermal_venture(input, true), 5);
    }

    #[test]
    fn test_hydrothermal_venture_empty() {
        assert_eq!(Day5::hydrothermal_venture(Vec::new(), true), 0);
    }

    #[test]
    fn test_hydrothermal_venture() {
        let input = utils::read_file("input/day5_2.txt".to_string());
        assert_eq!(Day5::hydrothermal_venture(input, true), 6687);
    }

    #[test]
    fn test_hydrothermal_venture_all_example() {
        let input = utils::read_file("input/day5_1.txt".to_string());
        assert_eq!(Day5::hydrothermal_venture(input, false), 12);
    }

    #[test]
    fn test_hydrothermal_venture_all_empty() {
        assert_eq!(Day5::hydrothermal_venture(Vec::new(), false), 0);
    }

    #[test]
    fn test_hydrothermal_venture_all() {
        let input = utils::read_file("input/day5_2.txt".to_string());
        assert_eq!(Day5::hydrothermal_venture(input, false), 19851);
    }
}
