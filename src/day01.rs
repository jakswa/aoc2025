pub fn part1(input: &str) -> String {
    let mut dial = 50;
    let mut zeros = 0;
    input.lines().for_each(|line| {
        let mut rot = line[1..].parse::<i64>().expect("valid input?");
        if &line[0..1] == "L" {
            rot = -rot;
        }
        dial = (dial + rot) % 100;
        if (dial < 0) {
            dial = 100 + dial;
        }
        if (dial == 0) {
            zeros += 1;
        }
    });
    zeros.to_string()
}

pub fn part2(input: &str) -> String {
    let dial = 50;
    return "TODO".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../input/day01_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "TODO");
    }
}
