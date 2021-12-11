struct Day8 {}

#[derive(Debug)]
struct Digit {
    flags: [i8; 7], // [a, b, c, d, e, f, g]
    value: i8,
    uniques: usize,
}

impl Digit {
    fn new(val: &str) -> Digit {
        let flags = Digit::str_into_flags(val);
        let uniques = flags.iter().filter(|&&f| f == 1).count();
        Digit {
            flags,
            value: Digit::flag_into_digit(flags, uniques),
            uniques,
        }
    }

    fn str_into_flags(val: &str) -> [i8; 7] {
        let mut flags = [0; 7];
        for ch in val.chars() {
            match ch {
                'a' => {
                    flags[0] = 1;
                }
                'b' => {
                    flags[1] = 1;
                }
                'c' => {
                    flags[2] = 1;
                }
                'd' => {
                    flags[3] = 1;
                }
                'e' => {
                    flags[4] = 1;
                }
                'f' => {
                    flags[5] = 1;
                }
                'g' => {
                    flags[6] = 1;
                }
                _ => {}
            }
        }

        flags
    }

    fn flag_into_digit(_flags: [i8; 7], uniques: usize) -> i8 {
        let mut digit: Option<i8> = None;
        match uniques {
            2 => {
                digit = Some(1);
            }
            4 => {
                digit = Some(4);
            }
            3 => {
                digit = Some(7);
            }
            7 => {
                digit = Some(8);
            }
            _ => {
                digit = Some(-1);
            }
        }

        digit.unwrap()
    }
}

impl Day8 {
    pub fn segment_search(lines: Vec<String>, all: bool) -> i32 {
        let mut count = 0;
        for line in lines.iter() {
            let end = line.split_once(" | ").unwrap().1;
            let segments = end.split(' ').collect::<Vec<&str>>();
            for seg in segments {
                let digit = Digit::new(seg);
                if !all {
                    match digit.value {
                        1 | 4 | 7 | 8 => {
                            count += 1;
                        }
                        _ => {}
                    }
                } else if digit.value != -1 {
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
    fn test_segment_search_example() {
        let input = utils::read_file_lines("input/day8_1.txt".to_string());
        assert_eq!(Day8::segment_search(input, false), 26);
    }

    #[test]
    fn test_segment_search_empty() {
        assert_eq!(Day8::segment_search(Vec::new(), false), 0);
    }

    #[test]
    fn test_segment_search() {
        let input = utils::read_file_lines("input/day8_2.txt".to_string());
        assert_eq!(Day8::segment_search(input, false), 416);
    }

    // #[test]
    // fn test_segment_search_all_example() {
    //     let input = utils::read_file_lines("input/day8_1.txt".to_string());
    //     assert_eq!(Day8::segment_search(input, true), 61229);
    // }

    // #[test]
    // fn test_segment_search_all_empty() {
    //     assert_eq!(Day8::segment_search(Vec::new(), true), 0);
    // }

    // #[test]
    // fn test_segment_search_all() {
    //     let input = utils::read_file_lines("input/day8_2.txt".to_string());
    //     assert_eq!(Day8::segment_search(input), 19851);
    // }
}
