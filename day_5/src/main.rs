use std::fs;
use std::ops::RangeInclusive;
use std::path::Iter;

fn make_range(range_str: &str) -> (u64, u64) {
    let range_numbers: Vec<&str> = range_str.split("-").collect();
    let bottom = range_numbers[0].parse::<u64>().unwrap();
    let top = range_numbers[1].parse::<u64>().unwrap();
    (bottom, top)
}

fn in_range(val: u64, range: &(u64, u64)) -> bool {
    if val >= range.0 && val <= range.1 {
        return true;
    }
    return false;
}

trait FreshRange {
    fn top(&mut self) -> Option<(u64, u64)>;
}

impl FreshRange for Vec<(u64, u64)> {
    fn top(&mut self) -> Option<(u64, u64)> {
        match self.len() {
            0 => None,
            n => Some(self[n - 1]),
        }
    }
}

fn part_1(contents: &String) -> i32 {
    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();
    let mut first_section = true;
    let mut count = 0;

    for line in contents.lines() {
        // println!("{line}");
        if line.len() == 0 {
            first_section = false;
            continue;
        }

        if first_section {
            fresh_ranges.push(make_range(line));
        } else {
            let num = line.parse::<u64>().unwrap();
            for range in &fresh_ranges {
                if in_range(num, &range) {
                    // println!("num {num} is in range {}:{}", range.0, range.1);
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

fn part_2(contents: &String) -> i32 {
    let mut count = 0;
    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();
    for line in contents.lines() {
        if line.len() == 0 {
            break;
        }
        // println!("{line}");
        let sub_range = make_range(line);
        fresh_ranges.push(sub_range);
        for sub_val in sub_range.0..=sub_range.1 {
            // println!("{sub_val} {count}");
            count += 1;
            for range in &fresh_ranges[..fresh_ranges.len().wrapping_sub(1)] {
                if in_range(sub_val, range) {
                    // println!("IN RANGE {sub_val} {} {}", range.0, range.1);
                    count -= 1;
                    break;
                }
            }
        }
    }
    count
}

fn main() {
    let filepath = "day_5/input/test_input.txt";
    let filepath = "day_5/input/input.txt";

    let contents = fs::read_to_string(filepath).expect("Something went wrong");
    let count_1 = part_1(&contents);
    let count_2 = part_2(&contents);

    println!("1 Final count: {count_1}");
    println!("2 Final count: {count_2}");
}
