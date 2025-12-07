use std::{ fs, ops::{ RangeInclusive } };

fn make_range(range_str: &str) -> RangeInclusive<u64> {
    let range_numbers: Vec<&str> = range_str.split("-").collect();
    let bottom = range_numbers[0].parse::<u64>().unwrap();
    let top = range_numbers[1].parse::<u64>().unwrap();
    bottom..=top
}

fn split_string(s: &str, chunk: usize) -> Vec<&str> {
    s.as_bytes()
        .chunks(chunk)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect()
}

fn is_elfish(number: u64) -> bool {
    let num_str = number.to_string();

    for chunk_size in (1..num_str.len()).rev() {
        if num_str.len() % chunk_size != 0 {
            continue;
        }
        let chunks = split_string(&num_str, chunk_size);
        if chunks.is_empty() {
            continue;
        }
        let check_seq = chunks[0];
        let mut ok = true;
        for seq in chunks {
            if check_seq != seq {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }

    return false;
}

fn main() {
    let filepath = "day_2/input/input.txt";

    let cur_dir = std::env::current_dir().unwrap();
    let cur_dir_str = cur_dir.to_str().unwrap();
    println!("dir: {cur_dir_str}");
    let contents = fs::read_to_string(filepath).expect("Something went wrong");

    // split contents by ','
    let separated_content = contents.split(",");

    // the sum of ID numbers that silly elf broke
    let mut sum_elfish = 0;

    for line in separated_content {
        let range = make_range(line);
        for num in range {
            if is_elfish(num) {
                sum_elfish += num;
            }
        }
    }
    println!("FINAL NUMBER: {sum_elfish}")
}
