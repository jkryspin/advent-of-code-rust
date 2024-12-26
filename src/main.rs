use aoc::util::ansi::*;
use aoc::util::parse::*;
use aoc::*;
use std::env::args;
use std::fs::read_to_string;
use std::iter::empty;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn main() {
    // Parse command line options
    let (year, day) = match args().nth(1) {
        Some(arg) => {
            let str = arg.as_str();
            let mut iter = str.iter_unsigned();
            (iter.next(), iter.next())
        }
        None => (None, None),
    };

    // Filter solutions
    let solutions: Vec<_> = empty()
        // .chain(year2015())
        // .chain(year2016())
        // .chain(year2017())
        // .chain(year2018())
        .chain(year2019())
        // .chain(year2020())
        // .chain(year2021())
        // .chain(year2022())
        // .chain(year2023())
        .chain(year2024())
        .filter(|solution| year.is_none_or(|y: u32| y == solution.year))
        .filter(|solution| day.is_none_or(|d: u32| d == solution.day))
        .collect();

    // Pretty print output for each solution.
    let mut duration = Duration::ZERO;

    for Solution { year, day, path, wrapper } in &solutions {
        if let Ok(data) = read_to_string(path) {
            let instant = Instant::now();
            let (part1, part2, part1_time, part2_time) = wrapper(data);
            duration += instant.elapsed();

            // Display part1_time
            // round to 1 decimal place
            if part1_time > 1000 {
                println!("    Part 1: {part1} ({:.1} ms)", part1_time as f32 / 1000f32);
            } else {
                println!("    Part 1: {part1} ({} µs)", part1_time);
            }

            // Display part2_time
            if part2_time > 1000 {
                println!("    Part 2: {part2} ({:.1} ms)", part2_time as f32 / 1000f32);
            } else {
                println!("    Part 2: {part2} ({} µs)", part2_time);
            }
        } else {
            eprintln!("{BOLD}{RED}{year} Day {day:02}{RESET}");
            eprintln!("    Missing input!");
            eprintln!("    Place input file in {BOLD}{WHITE}{}{RESET}", path.display());
        }
    }

    // Optionally print totals.
    if args().any(|a| a == "--totals") {
        println!("{BOLD}{YELLOW}⭐ {}{RESET}", 2 * solutions.len());
        println!("{BOLD}{WHITE}🕓 {} ms{RESET}", duration.as_millis());
    }
}

struct Solution {
    year: u32,
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> (String, String, u128, u128),
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year);
                let day = stringify!($day);
                let path = Path::new("input").join(year).join(day).with_extension("txt");

                let wrapper = |data: String| {
                    use $year::$day::*;

                    // print year and day
                    // let input = parse(&data);

                    // get duration
                    let instant = Instant::now();
                    let part1 = part1(&data);
                    let part1_time = instant.elapsed().as_micros();
                    let instant = Instant::now();
                    let part2 = part2(&data);
                    let part2_time = instant.elapsed().as_micros();



                    (part1.to_string(), part2.to_string(), part1_time, part2_time)
                };

                Solution { year: year.unsigned(), day: day.unsigned(), path, wrapper }
            },)*]
        }
    }
}

// run!(year2015
//     day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
//     day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
// );
//
// run!(year2016
//     day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
//     day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
// );
//
// run!(year2017
//     day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
//     day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
// );
//
// run!(year2018
//     day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
//     day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
// );
//
run!(year2019
    day01, day02, day03, day04, day05, day06
    // , day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    // day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);
//
// run!(year2020
//     day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
//     day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
// );
//
// run!(year2021
//     day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
//     day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
// );
//
// run!(year2022
//     day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
//     day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
// );
//
// run!(year2023
//     day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
//     day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
// );
//
run!(year2024
    day01,
    day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22,day23,  day24, day25
);
