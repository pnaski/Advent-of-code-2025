use std::{ fs, u64 };

fn square_size(x: (u64, u64), y: (u64, u64)) -> u64 {
    return (x.0.abs_diff(y.0) + 1) * (x.1.abs_diff(y.1) + 1);
}

fn conflicts_lines(
    one: (u64, u64),
    other: (u64, u64),
    x_lines: &Vec<(u64, u64, u64)>,
    y_lines: &Vec<(u64, u64, u64)>
) -> bool {
    let low = (one.0.min(other.0), one.1.min(other.1));
    let hi = (one.0.max(other.0), one.1.max(other.1));
    for line in y_lines {
        if line.0 > low.0 && line.0 < hi.0 {
            let line_min = line.1.min(line.2);
            let line_max = line.1.max(line.2);
            if (line_min <= low.1 && line_max > low.1) || (line_min < hi.1 && line_max >= hi.1) {
                return true;
            }
        }
    }
    for line in x_lines {
        if line.0 > low.1 && line.0 < hi.1 {
            let line_min = line.1.min(line.2);
            let line_max = line.1.max(line.2);
            if (line_min <= low.0 && line_max > low.0) || (line_min < hi.0 && line_max >= hi.0) {
                return true;
            }
        }
    }
    false
}

fn part_1(contents: &String) -> u64 {
    let mut ret = 0;
    let mut indexes: Vec<(u64, u64)> = Vec::new();

    for line in contents.lines() {
        let nums: Vec<u64> = line
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        indexes.push((nums[0], nums[1]));
    }

    for i in 0..indexes.len() {
        for j in 0..i {
            let size = square_size(indexes[i], indexes[j]);
            if size > ret {
                ret = size;
            }
        }
    }

    ret
}

fn part_2(contents: &String) -> u64 {
    let mut ret = 0;
    let mut indexes: Vec<(u64, u64)> = Vec::new();
    let mut x_lines: Vec<(u64, u64, u64)> = Vec::new();
    let mut y_lines: Vec<(u64, u64, u64)> = Vec::new();

    for line in contents.lines() {
        let nums: Vec<u64> = line
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        indexes.push((nums[0], nums[1]));
    }
    for i in 0..indexes.len() {
        let mut cur;
        let mut next;
        if i < indexes.len() - 1 {
            cur = indexes[i];
            next = indexes[i + 1];
        } else {
            cur = indexes[i];
            next = indexes[0];
        }
        if cur.0 == next.0 {
            y_lines.push((cur.0, cur.1, next.1));
        } else if cur.1 == next.1 {
            x_lines.push((cur.1, cur.0, next.0));
        } else {
            panic!("INVALID LINE");
        }
    }

    for i in 0..indexes.len() {
        for j in 0..i {
            if conflicts_lines(indexes[i], indexes[j], &x_lines, &y_lines) {
                continue;
            }
            let size = square_size(indexes[i], indexes[j]);
            if size > ret {
                ret = size;
            }
        }
    }

    ret
}

fn main() {
    let filepath = "day_9/input/test_input.txt";
    let filepath = "day_9/input/input.txt";

    let contents = fs::read_to_string(filepath).expect("Something went wrong");
    let out_1 = part_1(&contents);
    let out_2 = part_2(&contents);

    println!("1 Final out: {out_1}");
    println!("2 Final out: {out_2}");
}
