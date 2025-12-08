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

fn ranges_converge(lhs: (u64, u64), rhs: (u64, u64)) -> bool {
    if lhs.0 <= rhs.0 && lhs.1 >= rhs.1 {
        return true;
    }
    if lhs.0 >= rhs.0 && lhs.0 <= rhs.1 {
        return true;
    }
    if lhs.1 <= rhs.1 && lhs.1 >= rhs.0 {
        return true;
    }

    return false;
}

fn merge_ranges(lhs: (u64, u64), rhs: (u64, u64)) -> (u64, u64) {
    let mut ret_bot = 0;
    let mut ret_top = 0;

    if lhs.0 < rhs.0 {
        ret_bot = lhs.0;
    } else {
        ret_bot = rhs.0;
    }
    if lhs.1 > rhs.1 {
        ret_top = lhs.1;
    } else {
        ret_top = rhs.1;
    }

    (ret_bot, ret_top)
}

fn part_2(contents: &String) -> u64 {
    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();
    let mut count: u64 = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            break;
        }
        // println!("{line}");
        let mut merged = true;
        fresh_ranges.push(make_range(line));
        while merged {
            merged = false;
            let new = fresh_ranges.pop().unwrap();
            println!("Testing range {}:{}", new.0, new.1);
            println!("len: {}", fresh_ranges.len());
            if fresh_ranges.len() != 0 {
                for i in 0..fresh_ranges.len() {
                    let range = fresh_ranges[i];

                    if ranges_converge(new, range) {
                        println!("Testing against range {}:{}", range.0, range.1);
                        println!("Converged!");
                        let sub_range = merge_ranges(new, range);
                        fresh_ranges.remove(i);
                        fresh_ranges.push(sub_range);
                        merged = true;

                        break;
                    }
                }
            }

            if !merged {
                fresh_ranges.push(new);
            }
        }

        // fresh_ranges.push(sub_range);
        // for sub_val in sub_range.0..=sub_range.1 {
        //     // println!("{sub_val} {count}");
        //     count += 1;
        //     for range in &fresh_ranges[..fresh_ranges.len().wrapping_sub(1)] {
        //         if in_range(sub_val, range) {
        //             // println!("IN RANGE {sub_val} {} {}", range.0, range.1);
        //             count -= 1;
        //             break;
        //         }
        //     }
        // }
    }
    println!("OUT");
    for range in &fresh_ranges {
        println!("{}:{}", range.0, range.1);

        count += range.1 - range.0 + 1;
    }
    // count
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
