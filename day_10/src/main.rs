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

// fn solve_machine(machine: &Machine) -> Option<(u32, u32)> {
    
// }

fn solve(machines: &Vec<Machine>, check_joltage: bool) ->u32{
    let mut ret = 0;
    for machine in machines {
        // println!("{:?}", machine);
        let candidates = grab_initial_candidates(machine.target, &machine.buttons);
        let mut depth: u32 = 0;
        let buttons_count = machine.buttons.len();
        let mut visited_states: VecDeque<(u32, u32, Vec<i32>)> = VecDeque::new(); //(lights, visited_indexes)
        let mut skip_machine = false;
        for i in candidates {
            // println!("Candidate {i} {:0width$b}", machine.buttons[i], width=machine.width);
            if (machine.target ^ machine.buttons[i]) == 0 {
                ret += 1;
                skip_machine = true;
                break;
            }
            visited_states.push_back((
                machine.target ^ machine.buttons[i],
                1,
                dec_joltage(machine.buttons[i], &machine.joltages),
            ));
        }
        if skip_machine && !check_joltage {
            continue;
        }
        let mut completed = false;
        let mut deepest = 0;
        while !visited_states.is_empty() && !completed {
            let mut pushed_back = false;
            let state = visited_states.pop_front().unwrap();
            if state.1 > deepest {
                deepest = state.1;
                println!("{deepest}");
            }
            // println!("State: {:0l_width$b} {:0width$b}", state.0, state.1, l_width=machine.width, width = buttons_count);
            for i in 0..buttons_count {
                let new_light = state.0 ^ machine.buttons[i];
                let new_visited = state.1 + 1;
                // println!("  Visited: {:0l_width$b} {:0l_width$b} {:0width$b}", new_light, machine.buttons[i], new_visited,  l_width=machine.width, width=buttons_count);

                if new_light == 0 {
                    completed = true;
                    depth = new_visited;
                    break;
                    // println!("      COMPLETED: {:0width$b}", new_visited, width=machine.width);
                }

                if (state.0 & machine.buttons[i]) == 0 {
                    continue;
                }

                visited_states.push_back((
                    new_light,
                    new_visited,
                    dec_joltage(machine.buttons[i], &state.2),
                ));
            }
        }
        println!("DEPTH: {depth}");
        ret += depth;
    }
    ret
}

fn part_1(contents: &String) -> u32 {
    let machines = parse_input(contents);
    return solve(&machines, false);
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
        println!("{:?}", machine);
        let candidates = grab_initial_candidates(machine.target, &machine.buttons);
        let mut depth: u32 = 0;
        let buttons_count = machine.buttons.len();
        let mut visited_states: VecDeque<(u32, Vec<i32>)> = VecDeque::new(); //(lights, visited_indexes)

        let mut work_joltages = *machine.joltages;
        // println!("{:?}", work_joltages);
        loop {
            let a = best_fit_joltage(&work_joltages, &machine.buttons, machine.width);
            // println!("  {a}");
            if a == usize::MAX {
                break;
            }
            work_joltages = dec_joltage(machine.buttons[a], &work_joltages);
            depth += 1;
        }
        println!("{:?}", work_joltages);
        // println!("{depth}");
        // let mut decreased = false;
        for i in 0..machine.buttons.len() {
            visited_states.push_back((depth, work_joltages.clone()));
        }

        let mut completed = false;
        let mut deepest = 0;
        while !visited_states.is_empty() && !completed {
            let state = visited_states.pop_front().unwrap();
            if state.0 > deepest {
                deepest = state.0;
                println!("{deepest} {:?}", state.1);
            }
            for i in 0..buttons_count {
                let mut button_allowed = true;
                for (j, joltage) in state.1.iter().enumerate() {
                    if *joltage <= (0 as i32) && (machine.buttons[i] & (1 << j)) != 0 {
                        button_allowed = false;
                        break;
                    }
                }
                if button_allowed {
                    let new_joltage = dec_joltage(machine.buttons[i], &state.1);
                    let all_zeros = new_joltage.iter().all(|x| *x == 0);
                    if all_zeros {
                        completed = true;
                        depth = state.0 + 1;
                        break;
                    }
                    visited_states.push_back((state.0 + 1, new_joltage));
                }
                if completed {
                    break;
                }
            }
        }
        println!("DEPTH: {depth}");
        ret += depth;
    }
    ret
}

fn main() {
    let filepath = "day_10/input/test_input.txt";
    let filepath = "day_10/input/input.txt";

    let contents = fs::read_to_string(filepath).expect("Something went wrong");
    let out_1 = part_1(&contents);
    let out_2 = part_2(&contents);

    println!("1 Final out: {out_1}");
    println!("2 Final out: {out_2}");
}
