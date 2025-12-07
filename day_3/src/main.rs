use std::fs;

const JOLT_SIZE: usize = 12;
fn main() {
    let filepath = "day_3/input/test_input.txt";
    let filepath = "day_3/input/input.txt";

    let cur_dir = std::env::current_dir().unwrap();
    let cur_dir_str = cur_dir.to_str().unwrap();
    // println!("dir: {cur_dir_str}");
    let contents: String = fs::read_to_string(filepath).expect("Something went wrong");
    let mut sum: u64 = 0;

    for line in contents.lines() {
        let mut jolt_vals: [char; JOLT_SIZE] = ['0'; JOLT_SIZE];
        let mut jolt_indexes: [usize; JOLT_SIZE] = [0; JOLT_SIZE];
        let mut bot_limit: usize = 0;
        let mut top_limit: usize = line.len() - JOLT_SIZE + 1;

        for i in 0..jolt_vals.len() {
            // println!("1bot: {bot_limit} top: {top_limit}");
            for (index, character) in (&line[bot_limit..top_limit]).char_indices() {
                if character > jolt_vals[i] {
                    // println!("char is {character}");
                    jolt_vals[i] = character;
                    jolt_indexes[i] = index;
                }
            }
            // bot_limit = jolt_indexes;
            bot_limit = bot_limit + jolt_indexes[i] + 1;
            top_limit += 1;
            // println!("2bot: {bot_limit} top: {top_limit}");
        }

        for i in 0..jolt_vals.len() {
            let character = jolt_vals[i];
            // println!("i: {i} char:{character}");
            sum +=
                (character.to_digit(10).unwrap() as u64) *
                (10 as u64).pow((JOLT_SIZE - i - 1) as u32);
        }
    }
    println!("Sum is: {sum}");
}
