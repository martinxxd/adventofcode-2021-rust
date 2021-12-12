use std::collections::HashMap;

struct Day12 {}

impl Day12 {
    pub fn passage_pathing(lines: Vec<String>, multi: bool) -> i32 {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for line in lines.iter() {
            let split = line.split_once('-').unwrap();
            let (start, end) = (split.0, split.1);

            if map.contains_key(start) {
                map.get_mut(start).unwrap().push(end.to_string());
            } else {
                map.insert(start.to_string(), vec![end.to_string()]);
            }
            if map.contains_key(end) {
                map.get_mut(end).unwrap().push(start.to_string());
            } else {
                map.insert(end.to_string(), vec![start.to_string()]);
            }
        }

        let mut num_paths = 0;

        fn rec(map: &HashMap<String, Vec<String>>, pos: String, path: &mut Vec<String>, num_paths: &mut i32, multi: bool) {
            if pos == "end" {
                *num_paths += 1;
                return;
            }

            fn invalid_path(next_path: &String, path: &Vec<String>, multi: bool) -> bool {
                let mut count: HashMap<&String, i32> = HashMap::new();

                for p in path {
                    *count.entry(p).or_insert(0) += 1;
                }
                *count.entry(next_path).or_insert(0) += 1;

                let mut seen_twice = false;
                for (pos, count) in count.iter() {
                    if !pos.to_lowercase().eq(*pos) {
                        continue;
                    }

                    if multi {
                        if *count == 1 {
                            continue;
                        } else if *count == 2 && !seen_twice {
                            seen_twice = true;
                            continue;
                        } else if *count == 2 && seen_twice {
                            return true;
                        } else if *count > 2 {
                            return true;
                        }
                    } else if *count > 1 {
                        return true;
                    }
                }

                false
            }

            if let Some(next) = map.get(&pos) {
                for next_path in next {
                    if next_path.eq("start") || invalid_path(next_path, path, multi) {
                        continue;
                    }
                    path.push(next_path.clone());
                    rec(map, next_path.clone(), path, num_paths, multi);
                    path.pop();
                }
            }
        }

        rec(&map, "start".to_string(), &mut vec![], &mut num_paths, multi);

        num_paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_passage_pathing_example1() {
        let input = utils::read_file_lines("input/day12_1.txt".to_string());
        assert_eq!(Day12::passage_pathing(input, false), 10);
    }

    #[test]
    fn test_passage_pathing_example2() {
        let input = utils::read_file_lines("input/day12_2.txt".to_string());
        assert_eq!(Day12::passage_pathing(input, false), 19);
    }

    #[test]
    fn test_passage_pathing_example3() {
        let input = utils::read_file_lines("input/day12_3.txt".to_string());
        assert_eq!(Day12::passage_pathing(input, false), 226);
    }

    #[test]
    fn test_passage_pathing_example4() {
        let input = utils::read_file_lines("input/day12_4.txt".to_string());
        assert_eq!(Day12::passage_pathing(input, false), 4104);
    }

    #[test]
    fn test_passage_pathing_multi_example1() {
        let input = utils::read_file_lines("input/day12_1.txt".to_string());
        assert_eq!(Day12::passage_pathing(input, true), 36);
    }

    #[test]
    fn test_passage_pathing_multi_example2() {
        let input = utils::read_file_lines("input/day12_2.txt".to_string());
        assert_eq!(Day12::passage_pathing(input, true), 103);
    }

    // TODO: Improve performance
    // #[test]
    // fn test_passage_pathing_multi_example3() {
    //     let input = utils::read_file_lines("input/day12_3.txt".to_string());
    //     assert_eq!(Day12::passage_pathing(input, true), 3509);
    // }

    // TODO: Improve performance
    // #[test]
    // fn test_passage_pathing_multi_example4() {
    //     let input = utils::read_file_lines("input/day12_4.txt".to_string());
    //     assert_eq!(Day12::passage_pathing(input, true), 119760);
    // }
}
