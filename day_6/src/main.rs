use std::{ default, fs };

fn part_1(contents: &String) -> u64 {
    let mut ret = 0;
    let mut ops: Vec<char> = Vec::new();
    let mut calculated: Vec<u64> = Vec::new();

    for line in contents.lines().rev() {
        if calculated.len() != line.len() {
            calculated.resize(line.len(), 0);
        }
        let stripped: Vec<&str> = line.split_whitespace().collect();
        for (i, sub) in stripped.iter().enumerate() {
            if *sub == String::from("*") || *sub == String::from("+") {
                let in_chars: Vec<char> = sub.chars().collect();
                ops.push(in_chars[0]);
                continue;
            }
            // println!("{sub}");
            let number = sub.parse::<u64>().unwrap();
            match ops[i] {
                '*' => {
                    if calculated[i] == 0 {
                        calculated[i] = number;
                    } else {
                        calculated[i] *= number;
                    }
                }
                '+' => {
                    calculated[i] += number;
                }
                _ => println!("Op is wrong {}", ops[i]),
            }
        }
    }
    for val in calculated {
        ret += val;
    }
    ret
}

fn part_2(contents: &String) -> u64 {
    let mut ret = 0;
    let mut ops: Vec<char> = Vec::new();
    let mut numbers: Vec<Vec<&str>> = Vec::new();
    let mut pows: Vec<usize> = Vec::new();
    let signs: Vec<&str> = contents.lines().last().unwrap().split_whitespace().collect();
    let col_count = signs.len();
    let line_count: usize = contents.lines().count() - 1;

    let mut digits: Vec<Vec<Vec<char>>> = Vec::new();
    digits.resize(col_count, Vec::new());
    for i in 0..digits.len() {
        digits[i].resize(line_count, Vec::new());
    }

    let mut line_cnt: usize = 0;
    let mut col_index: usize = 0;

    for line in contents.lines() {
        if line.starts_with("*") || line.starts_with("+") {
            continue;
        }
        let mut num_index: usize = 0;
        for character in line.chars() {
            digits[col_index][line_cnt][i];
        }
        line_cnt += 1;
    }

    ret
}

fn main() {
    let filepath = "day_6/input/test_input.txt";
    let filepath = "day_6/input/input.txt";

    let contents = fs::read_to_string(filepath).expect("Something went wrong");
    let out_1 = part_1(&contents);
    let out_2 = part_2(&contents);

    println!("1 Final out: {out_1}");
    println!("2 Final out: {out_2}");
}
