use std::{ collections::VecDeque, fs };

#[derive(Debug)]
struct Machine {
    width: usize,
    target: u32,
    buttons: Box<Vec<u32>>,
    joltages: Box<Vec<i32>>,
}

fn parse_input(contents: &String) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    for line in contents.lines() {
        let mut cur_machine = Machine {
            width: 0,
            target: 0,
            buttons: Box::new(Vec::new()),
            joltages: Box::new(Vec::new()),
        };
        // println!("{line} ");
        let sections: Vec<&str> = line.split(' ').collect();
        cur_machine.width = sections[0].len() - 2;
        for section in sections {
            if section.starts_with('[') {
                let section_chars = section[1..section.len() - 1].chars();
                for (i, c) in section_chars.enumerate() {
                    if c == '#' {
                        cur_machine.target |= 1 << i;
                    }
                }
            } else if section.starts_with('(') {
                let mut button: u32 = 0;
                for number in section[1..section.len() - 1].split(',') {
                    button |= 1 << number.parse::<u32>().unwrap();
                }
                cur_machine.buttons.push(button);
            } else if section.starts_with('{') {
                for number in section[1..section.len() - 1].split(',') {
                    cur_machine.joltages.push(number.parse::<i32>().unwrap());
                }
            } else {
                panic!("Invalid section");
            }
        }
        // println!("{:?}", cur_machine);
        machines.push(cur_machine);
    }
    machines
}

fn grab_initial_candidates(target: u32, buttons: &Vec<u32>) -> Vec<usize> {
    let mut ret = Vec::new();

    for (i, button) in buttons.iter().enumerate() {
        if (button & target) != 0 {
            ret.push(i);
        }
    }

    ret
}

fn dec_joltage(button: u32, joltages: &Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::new();
    for i in 0..joltages.len() {
        if (button & (1 << i)) != 0 {
            ret.push(joltages[i] - 1);
        } else {
            ret.push(joltages[i]);
        }
    }
    ret
}

fn solve_machine(
    target: u32,
    buttons: &Vec<u32>,
    want_all: bool,
    width: usize
) -> Option<(u32, Vec<u32>)> {
    let candidates = grab_initial_candidates(target, &buttons);
    let mut depth: u32 = 0;
    let buttons_count = buttons.len();
    let mut visited_states: VecDeque<(u32, u32)> = VecDeque::new(); //(lights, visited_indexes)
    let mut valid_buttons = Vec::new();
    let mut skip_machine = false;
    for i in candidates {
        // println!("Candidate {i} {:0width$b}", buttons[i], width=width);
        if (target ^ buttons[i]) == 0 {
            depth = 1;
            skip_machine = true;
            valid_buttons.push(1 << i);
            break;
        }
        visited_states.push_back((target ^ buttons[i], 1 << i));
    }
    if skip_machine && !want_all {
        return Some((1, valid_buttons));
    }
    let mut completed = false;
    while !visited_states.is_empty() && !completed {
        let state = visited_states.pop_front().unwrap();

        for i in 0..buttons_count {
            let new_light = state.0 ^ buttons[i];
            let new_visited = state.1 | (1 << i);

            if new_light == 0 {
                completed = true;
                depth = new_visited.count_ones();
                if !valid_buttons.contains(&new_visited) {
                    valid_buttons.push(new_visited);
                }
                if !want_all {
                    break;
                }
                if depth < (width as u32) {
                    completed = false;
                }
                continue;
            }

            if (state.0 & buttons[i]) == 0 {
                continue;
            }

            if (state.1 & (1 << i)) == 1 {
                continue;
            }

            visited_states.push_back((new_light, new_visited));
        }
    }
    match valid_buttons.len() {
        0 => None,
        _ => Some((depth, valid_buttons)),
    }
}

fn part_1(contents: &String) -> u32 {
    let machines = parse_input(contents);
    let mut ret = 0;
    for machine in machines {
        // println!("{:?}", machine);
        let (depth, _) = solve_machine(machine.target, &machine.buttons, false, 0).unwrap();
        ret += depth;
    }
    ret
}

fn best_fit_joltage(joltages: &Vec<i32>, buttons: &Vec<u32>, joltage_threshold: usize) -> usize {
    let mut best_match = usize::MAX;
    let mut best_counter = 0;
    for (i, button) in buttons.iter().enumerate() {
        // println!("      i{i}, {:b}", button);
        let mut counter = 0;
        let mut allowed = true;
        for (j, joltage) in joltages.iter().enumerate() {
            // print!("      j{j} {} {}", *joltage, button & (1<<j));
            if *joltage > (joltage_threshold as i32) && (button & (1 << j)) != 0 {
                // println!("  entered");
                counter += 1;
            }
            if *joltage < (joltage_threshold as i32) && (button & (1 << j)) != 0 {
                allowed = false;
                break;
            }
        }
        if counter > best_counter && allowed {
            // println!("          best match");
            best_counter = counter;
            best_match = i;
        }
    }

    best_match
}

fn part_2(contents: &String) -> u32 {
    let machines = parse_input(contents);
    let mut ret = 0;

    for machine in machines {
        let mut joltages = *machine.joltages;
        let mut j_target = 0;
        let mut ret_depth = 0;
        for (i, joltage) in joltages.iter().enumerate() {
            if joltage % 2 == 1 {
                j_target |= 1 << i;
            }
        }
        loop {
            let (depth, valid_solutions) = solve_machine(
                j_target,
                &machine.buttons,
                true,
                machine.width
            ).unwrap();
            println!("{:?}", valid_solutions);
            for j in 0..machine.buttons.len() {
                if (valid_solutions[0] & (1 << j)) != 0 {
                    ret_depth += 1;
                    for i in 0..machine.width {
                        if (machine.buttons[j] & (1 << i)) != 0 {
                            joltages[i] -= 1;
                        }
                    }
                }
            }
            println!("{:?}", joltages);
            if joltages.iter().all(|x| *x == 0) {
                println!("Completed");
                break;
            }
        }
        ret += ret_depth;
    }
    ret
}

fn main() {
    let filepath = "day_10/input/input.txt";
    let filepath = "day_10/input/test_input.txt";

    let contents = fs::read_to_string(filepath).expect("Something went wrong");
    let out_1 = part_1(&contents);
    println!("1 Final out: {out_1}");
    let out_2 = part_2(&contents);

    println!("2 Final out: {out_2}");
}
