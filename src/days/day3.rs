struct Day3 {}

impl Day3 {
    pub fn binary_diagnostic(lines: Vec<String>) -> i32 {
        let mut bits = Vec::<(i32, i32)>::new();

        for line in lines {
            line.chars().enumerate().for_each(|(i, b)| {
                if i >= bits.len() {
                    bits.push((0, 0));
                }

                if b == '0' {
                    bits[i].0 += 1;
                } else {
                    bits[i].1 += 1;
                }
            });
        }

        let mut gamma = 0;
        let mut epsilon = 0;
        bits.iter().enumerate().for_each(|(i, (a, b))| {
            if a < b {
                gamma |= 1 << (bits.len() - i - 1)
            }
            if a > b {
                epsilon |= 1 << (bits.len() - i - 1)
            }
        });

        gamma * epsilon
    }

    pub fn binary_diagnostic_life_support(lines: Vec<String>) -> i32 {
        let mut oxygen: u32 = 0;
        let mut scrubber: u32 = 0;

        let mut curr_oxygen = String::new();
        let mut curr_scrubber = String::new();
        let l = lines.first().unwrap_or(&"".to_string()).len();
        for i in 0..l {
            let mut count = (0, 0);
            for j in 0..lines.len() {
                if !lines[j].starts_with(&curr_oxygen) {
                    continue;
                }

                if lines[j].chars().nth(i).unwrap() == '0' {
                    count.0 += 1;
                } else {
                    count.1 += 1;
                }
            }

            if count.0 <= count.1 {
                oxygen |= 1 << (l - i - 1);
                curr_oxygen += "1";
            } else {
                curr_oxygen += "0";
            }

            let mut count = (0, 0);
            for j in 0..lines.len() {
                if !lines[j].starts_with(&curr_scrubber) {
                    continue;
                }

                if lines[j].chars().nth(i).unwrap() == '0' {
                    count.0 += 1;
                } else {
                    count.1 += 1;
                }
            }

            if count.1 == 0 || (count.0 > 0 && count.0 <= count.1) {
                curr_scrubber += "0";
            } else {
                scrubber |= 1 << (l - i - 1);
                curr_scrubber += "1";
            }
        }

        (oxygen * scrubber) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_binary_diagnostic_example() {
        let input = utils::read_file("input/day3_1.txt".to_string());
        assert_eq!(Day3::binary_diagnostic(input), 198);
    }

    #[test]
    fn test_binary_diagnostic_empty() {
        assert_eq!(Day3::binary_diagnostic(Vec::new()), 0);
    }

    #[test]
    fn test_binary_diagnostic() {
        let input = utils::read_file("input/day3_2.txt".to_string());
        assert_eq!(Day3::binary_diagnostic(input), 2954600);
    }

    #[test]
    fn test_binary_diagnostic_life_support_example() {
        let input = utils::read_file("input/day3_1.txt".to_string());
        assert_eq!(Day3::binary_diagnostic_life_support(input), 230);
    }

    #[test]
    fn test_binary_diagnostic_life_support_empty() {
        assert_eq!(Day3::binary_diagnostic_life_support(Vec::new()), 0);
    }

    #[test]
    fn test_binary_diagnostic_life_support() {
        let input = utils::read_file("input/day3_2.txt".to_string());
        assert_eq!(Day3::binary_diagnostic_life_support(input), 1662846);
    }
}
