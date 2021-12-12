struct Day8 {}

#[derive(Debug)]
struct Digit {
    flags: [i8; 7], // [a, b, c, d, e, f, g]
    value: Option<i8>,
    options: Vec<i8>,
    uniques: usize,
}

impl Digit {
    fn new(val: &str) -> Digit {
        let flags = Digit::str_into_flags(val);
        let uniques = flags.iter().filter(|&&f| f == 1).count();
        let (value, options) = Digit::flag_into_digit(flags, uniques);
        Digit { flags, value, options, uniques }
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

    fn flag_into_digit(_flags: [i8; 7], uniques: usize) -> (Option<i8>, Vec<i8>) {
        let mut digit: Option<i8> = None;
        let mut options: Vec<i8> = Vec::<i8>::new();
        match uniques {
            2 => {
                digit = Some(1);
            }
            3 => {
                digit = Some(7);
            }
            4 => {
                digit = Some(4);
            }
            5 => {
                options = vec![2, 3, 5];
            }
            6 => {
                options = vec![0, 6, 9];
            }
            7 => {
                digit = Some(8);
            }
            _ => {
                digit = Some(-1);
            }
        }

        if digit.is_some() {
            options.push(digit.unwrap());
        }
        (digit, options)
    }
}

impl Day8 {
    pub fn segment_search(lines: Vec<String>) -> i32 {
        let mut count = 0;
        for line in lines.iter() {
            let end = line.split_once(" | ").unwrap().1;
            let segments = end.split(' ').collect::<Vec<&str>>();
            for seg in segments {
                let digit = Digit::new(seg);
                if digit.value.is_some() {
                    match digit.value.unwrap() {
                        1 | 4 | 7 | 8 => {
                            count += 1;
                        }
                        _ => {}
                    }
                }
            }
        }

        count
    }

    pub fn segment_search_all(lines: Vec<String>) -> i32 {
        let mut count = 0;
        for line in lines.iter() {
            let split = line.split_once(" | ").unwrap();

            let mut segments = split.0.split(' ').collect::<Vec<&str>>();
            segments.append(&mut split.1.split(' ').collect::<Vec<&str>>());

            for seg in segments {
                let digit = Digit::new(seg);
                if digit.value.is_some() {
                    match digit.value.unwrap() {
                        1 | 4 | 7 | 8 => {
                            count += 1;
                        }
                        _ => {}
                    }
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
        assert_eq!(Day8::segment_search(input), 26);
    }

    #[test]
    fn test_segment_search_empty() {
        assert_eq!(Day8::segment_search(Vec::new()), 0);
    }

    #[test]
    fn test_segment_search() {
        let input = utils::read_file_lines("input/day8_2.txt".to_string());
        assert_eq!(Day8::segment_search(input), 416);
    }
}
