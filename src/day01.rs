pub fn part1(input: &str) -> String {
    let mut dial = 50;
    let mut zeros = 0;
    input.lines().for_each(|line| {
        let mut rot = line[1..].parse::<i64>().expect("valid input?");
        if &line[0..1] == "L" {
            rot = -rot;
        }
        dial = (dial + rot) % 100;
        if dial < 0 {
            dial = 100 + dial;
        }
        if dial == 0 {
            zeros += 1;
        }
    });
    zeros.to_string()
}

pub fn part2(input: &str) -> String {
    let mut dial = 50;
    let mut zeros = 0;
    input.lines().for_each(|line| {
        let start_zero = dial == 0;
        let mut rot = line[1..].parse::<i64>().expect("valid input?");
        if &line[0..1] == "L" {
            rot = -rot;
        }
        dial = dial + rot;
        if dial == 0 {
            zeros += 1;
        } else if dial < 0 {
            zeros += (dial / 100).abs() + 1;
            if start_zero {
                zeros -= 1;
            }
            dial = dial % 100;
            if dial < 0 {
                dial = 100 + dial;
            }
        } else if dial > 99 {
            zeros += dial / 100;
            dial = dial % 100;
        }
        println!("for {} done by {} its {}", dial, rot, zeros);
    });
    zeros.to_string()
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
        assert_eq!(part2(TEST_INPUT), "6");
    }

    #[test]
    fn test_part2_R1000() {
        assert_eq!(part2("L50\nR1000"), "11");
    }

    #[test]
    fn test_part2_L1000() {
        assert_eq!(part2("R50\nL1000"), "11");
    }
}
