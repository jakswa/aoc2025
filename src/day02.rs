pub fn part1(input: &str) -> String {
    input
        .lines()
        .next()
        .expect("one line input")
        .split([',', '\n'])
        .filter_map(|id_range| {
            let (left, right) = id_range.split_once("-").expect("string with dash for sure");
            let left64 = left.parse::<u64>().expect("number here for sure");
            let right64 = right.parse::<u64>().expect("number here too");
            let ids_matching: u64 = (left64..=right64)
                .filter_map(|id| {
                    let id_str = id.to_string();
                    let len = id_str.len();
                    if len % 2 == 0 && id_str[0..len / 2] == id_str[len / 2..len] {
                        return Some(id);
                    }
                    None
                })
                .sum();
            if ids_matching > 0 {
                return Some(ids_matching);
            } else {
                return None;
            }
        })
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .next()
        .expect("one line input")
        .split([',', '\n'])
        .filter_map(|id_range| {
            let (left, right) = id_range.split_once("-").expect("string with dash for sure");
            let left64 = left.parse::<u64>().expect("number here for sure");
            let right64 = right.parse::<u64>().expect("number here too");
            let ids_matching: u64 = (left64..=right64)
                .filter_map(|id| {
                    let id_str = id.to_string();
                    let len = id_str.len();
                    if (1..=(len / 2))
                        .any(|sub_ind| id_str.split(&id_str[0..sub_ind]).collect::<String>() == "")
                    {
                        return Some(id);
                    }
                    None
                })
                .sum();
            if ids_matching > 0 {
                return Some(ids_matching);
            } else {
                return None;
            }
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../input/day02_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "1227775554");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "4174379265");
    }
}
