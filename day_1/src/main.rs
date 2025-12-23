use std::fs;

fn main() {
    let filepath = "day_1/input/input.txt";

    let contents = fs::read_to_string(filepath).expect("Something went wrong");
    let mut dial = 50;
    let mut click = 0;
    let mut pre_click = 0;

    for line in contents.lines() {
        pre_click = click;
        let mut dial_was_zero = false;
        let subtract = false;
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let num_str: String = chars.take(6).collect();
        let num: i32 = num_str.parse::<i32>().unwrap();

        if dir == 'L' {
            if dial == 0 {
                dial += 100;
            }
            dial -= num;
        } else {
            dial += num;
        }
        // normal, or "end of circle" click
        if dial == 0 || (dial < 0 && dial % 100 == 0) {
            click += 1;
        }
        // right rotation click counter
        while dial >= 100 {
            click += 1;
            dial -= 100;
        }
        // left rotation click counter
        while dial < 0 {
            click += 1;
            dial += 100;
        }

        // printing to debug whether code works as intended
        let clicked = pre_click < click;
        print!("{dir}: {num} | {dial}");
        if clicked {
            println!(" click {click}");
        } else {
            println!("");
        }
    }
    println!("FINAL RESULT: {dial} {click}");
}
