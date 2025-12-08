use std::fs::{ self };

fn adjust_to_bounds(val: usize, max_val: usize) -> usize {
    if val >= max_val {
        return max_val;
    }
    val
}

fn count_in_sub_grid(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let bot_i = i.saturating_sub(1);
    let top_i = adjust_to_bounds(i + 1, grid.len() - 1);
    let bot_j = j.saturating_sub(1);
    let top_j = adjust_to_bounds(j + 1, grid[0].len() - 1);
    let mut count = 0;
    for x in bot_i..top_i + 1 {
        for y in bot_j..top_j + 1 {
            if x == i && y == j {
                continue;
            }
            if grid[x][y] == '@' || grid[x][y] == 'x' {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let filepath = "input/test_input";
    let filepath = "input/actual_input";
    let contents = String::from_utf8(fs::read(filepath).unwrap()).unwrap();
    let mut grid_i: usize = 0;
    let mut final_count = 0;
    let mut line_length: usize = 0;

    for line in contents.lines() {
        line_length = line.len();
        grid.push(Vec::new());
        for (j, character) in line.char_indices() {
            grid[grid_i].push(character);
        }
        grid_i += 1;
    }
    let mut removed = true;

    while removed {
        removed = false;

        for i in 0..grid.len() {
            for j in 0..line_length {
                // print!("{}", grid[i][j]);
                if grid[i][j] == '.' {
                    continue;
                }
                let count = count_in_sub_grid(&grid, i, j);
                if count < 4 {
                    final_count += 1;
                    grid[i][j] = 'x';
                }
                // println!("{i}:{j} | {count}");
            }
            // println!("");
        }
        for i in 0..grid.len() {
            for j in 0..line_length {
                // print!("{}", grid[i][j]);
                if grid[i][j] == 'x' {
                    grid[i][j] = '.';
                    removed = true;
                }
            }
            // println!("");
        }
    }

    println!("FINAL: {final_count}");
}
