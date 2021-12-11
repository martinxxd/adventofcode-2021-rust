struct Day7 {}

impl Day7 {
    pub fn treachery_of_whales(input: String, acc: bool) -> i32 {
        if input.is_empty() {
            return 0;
        }

        let heights: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

        fn calc_fuel(heights: &Vec<i32>, target_height: i32, acc: bool) -> i32 {
            let mut fuel = 0;
            for height in heights.iter() {
                let steps = i32::abs(height - target_height);
                if acc {
                    let mut step = 1;
                    for i in 0..steps {
                        fuel += step;
                        step += 1;
                    }
                } else {
                    fuel += i32::abs(height - target_height);
                }
            }

            fuel
        }

        let (lo, hi) = (*heights.iter().min().unwrap(), *heights.iter().max().unwrap());
        let mut min_fuel = i32::MAX;

        for target in lo..hi {
            let fuel = calc_fuel(&heights, target, acc);
            min_fuel = i32::min(min_fuel, fuel);
        }

        min_fuel
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_treachery_of_whales_example() {
        let input = utils::read_file("input/day7_1.txt".to_string());
        assert_eq!(Day7::treachery_of_whales(input, false), 37);
    }

    #[test]
    fn test_treachery_of_whales_empty() {
        assert_eq!(Day7::treachery_of_whales("".to_string(), false), 0);
    }

    #[test]
    fn test_treachery_of_whales() {
        let input = utils::read_file("input/day7_2.txt".to_string());
        assert_eq!(Day7::treachery_of_whales(input, false), 342641);
    }

    #[test]
    fn test_treachery_of_whales_all_example() {
        let input = utils::read_file("input/day7_1.txt".to_string());
        assert_eq!(Day7::treachery_of_whales(input, true), 168);
    }

    #[test]
    fn test_treachery_of_whales_all_empty() {
        assert_eq!(Day7::treachery_of_whales("".to_string(), true), 0);
    }

    // TODO: Improve performance
    // #[test]
    // fn test_treachery_of_whales_all() {
    //     let input = utils::read_file("input/day7_2.txt".to_string());
    //     assert_eq!(Day7::treachery_of_whales(input, true), 93006301);
    // }
}
