struct Day1 {}

impl Day1 {
    pub fn sonar_sweep(input: Vec<String>) -> i32 {
        let sonars: Vec<i32> = input
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut prev = None;
        let mut count = 0;
        for sonar in sonars {
            if let Some(prev) = prev {
                if prev < sonar {
                    count += 1;
                }
            }
            prev = Some(sonar);
        }

        count
    }

    pub fn sonar_sweep_window(input: Vec<String>) -> i32 {
        let sonars: Vec<i32> = input
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        // build rolling window
        let mut window_sum = 0;
        let mut window_size = 0;
        let mut sonars_window = Vec::new();
        for i in 0..sonars.len() {
            window_sum += sonars[i];
            window_size += 1;
            if window_size == 3 {
                sonars_window.push(window_sum.to_string());
            } else if window_size > 3 {
                window_sum -= sonars[i - 3];
                sonars_window.push(window_sum.to_string());
            }
        }

        Day1::sonar_sweep(sonars_window)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_sonar_sweep_example() {
        let input = utils::read_file_lines("input/day1_1.txt".to_string());
        assert_eq!(Day1::sonar_sweep(input), 7);
    }

    #[test]
    fn test_sonar_sweep_empty() {
        assert_eq!(Day1::sonar_sweep(Vec::new()), 0);
    }

    #[test]
    fn test_sonar_sweep() {
        let input = utils::read_file_lines("input/day1_2.txt".to_string());
        assert_eq!(Day1::sonar_sweep(input), 1616);
    }

    #[test]
    fn test_sonar_sweep_window_example() {
        let input = utils::read_file_lines("input/day1_1.txt".to_string());
        assert_eq!(Day1::sonar_sweep_window(input), 5);
    }

    #[test]
    fn test_sonar_sweep_window_empty() {
        assert_eq!(Day1::sonar_sweep_window(Vec::new()), 0);
    }

    #[test]
    fn test_sonar_sweep_window() {
      let input = utils::read_file_lines("input/day1_2.txt".to_string());
      assert_eq!(Day1::sonar_sweep_window(input), 1645);
    }
}
