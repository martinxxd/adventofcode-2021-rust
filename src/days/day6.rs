struct Day6 {}

impl Day6 {
    pub fn lanternfish(input: String, days: i32) -> i64 {
        if input.is_empty() {
            return 0;
        }

        let mut fishs = input.split(',').fold([0; 9], |mut map, n| {
            map[n.parse::<usize>().unwrap()] += 1;
            map
        });

        (1..days as usize).for_each(|day| fishs[(day + 7) % 9] += fishs[day % 9]);

        fishs.iter().sum::<i64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_lanternfish_example() {
        let input = utils::read_file("input/day6_1.txt".to_string());
        assert_eq!(Day6::lanternfish(input, 80), 5934);
    }

    #[test]
    fn test_lanternfish_empty() {
        assert_eq!(Day6::lanternfish("".to_string(), 80), 0);
    }

    #[test]
    fn test_lanternfish() {
        let input = utils::read_file("input/day6_2.txt".to_string());
        assert_eq!(Day6::lanternfish(input, 80), 380758);
    }

    #[test]
    fn test_lanternfish_all_example() {
        let input = utils::read_file("input/day6_1.txt".to_string());
        assert_eq!(Day6::lanternfish(input, 256), 26984457539);
    }

    #[test]
    fn test_lanternfish_all_empty() {
        assert_eq!(Day6::lanternfish("".to_string(), 256), 0);
    }

    #[test]
    fn test_lanternfish_all() {
        let input = utils::read_file("input/day6_2.txt".to_string());
        assert_eq!(Day6::lanternfish(input, 256), 1710623015163);
    }
}
