use std::fs;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day> [part]");
        eprintln!("Example: cargo run -- 1");
        eprintln!("Example: cargo run -- 1 2");
        return;
    }

    let day: u8 = args[1].parse().expect("Invalid day number");
    let part = args.get(2).and_then(|s| s.parse().ok());

    let input_file = format!("input/day{:02}.txt", day);
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", input_file));

    match (day, part) {
        (1, Some(1)) | (1, None) => {
            println!("Day 1, Part 1: {}", aoc2025::day01::part1(&input));
        }
        (1, Some(2)) => {
            println!("Day 1, Part 2: {}", aoc2025::day01::part2(&input));
        }
        (2, Some(1)) | (2, None) => {
            println!("Day 2, Part 1: {}", aoc2025::day02::part1(&input));
        }
        (2, Some(2)) => {
            println!("Day 2, Part 2: {}", aoc2025::day02::part2(&input));
        }
        (3, Some(1)) | (3, None) => {
            println!("Day 3, Part 1: {}", aoc2025::day03::part1(&input));
        }
        (3, Some(2)) => {
            println!("Day 3, Part 2: {}", aoc2025::day03::part2(&input));
        }
        (4, Some(1)) | (4, None) => {
            println!("Day 4, Part 1: {}", aoc2025::day04::part1(&input));
        }
        (4, Some(2)) => {
            println!("Day 4, Part 2: {}", aoc2025::day04::part2(&input));
        }
        (5, Some(1)) | (5, None) => {
            println!("Day 5, Part 1: {}", aoc2025::day05::part1(&input));
        }
        (5, Some(2)) => {
            println!("Day 5, Part 2: {}", aoc2025::day05::part2(&input));
        }
        (6, Some(1)) | (6, None) => {
            println!("Day 6, Part 1: {}", aoc2025::day06::part1(&input));
        }
        (6, Some(2)) => {
            println!("Day 6, Part 2: {}", aoc2025::day06::part2(&input));
        }
        (7, Some(1)) | (7, None) => {
            println!("Day 7, Part 1: {}", aoc2025::day07::part1(&input));
        }
        (7, Some(2)) => {
            println!("Day 7, Part 2: {}", aoc2025::day07::part2(&input));
        }
        (8, Some(1)) | (8, None) => {
            println!("Day 8, Part 1: {}", aoc2025::day08::part1(&input));
        }
        (8, Some(2)) => {
            println!("Day 8, Part 2: {}", aoc2025::day08::part2(&input));
        }
        (9, Some(1)) | (9, None) => {
            println!("Day 9, Part 1: {}", aoc2025::day09::part1(&input));
        }
        (9, Some(2)) => {
            println!("Day 9, Part 2: {}", aoc2025::day09::part2(&input));
        }
        (10, Some(1)) | (10, None) => {
            println!("Day 10, Part 1: {}", aoc2025::day10::part1(&input));
        }
        (10, Some(2)) => {
            println!("Day 10, Part 2: {}", aoc2025::day10::part2(&input));
        }
        (11, Some(1)) | (11, None) => {
            println!("Day 11, Part 1: {}", aoc2025::day11::part1(&input));
        }
        (11, Some(2)) => {
            println!("Day 11, Part 2: {}", aoc2025::day11::part2(&input));
        }
        (12, Some(1)) | (12, None) => {
            println!("Day 12, Part 1: {}", aoc2025::day12::part1(&input));
        }
        (12, Some(2)) => {
            println!("Day 12, Part 2: {}", aoc2025::day12::part2(&input));
        }
        _ => eprintln!("Day {} part {:?} not implemented", day, part),
    }
}
