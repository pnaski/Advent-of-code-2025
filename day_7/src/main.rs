use std::fs;
use std::process::Command;

struct BinaryNode<T> {
    value: Option<T>,
    left: Option<Box<BinaryNode<T>>>,
    right: Option<Box<BinaryNode<T>>>,
}
impl<T> BinaryNode<T> {
    pub fn new() -> Self {
        BinaryNode {
            value: None,
            left: None,
            right: None,
        }
    }
    pub fn from_value(value: T) -> Self {
        BinaryNode {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    pub fn left(&self) -> Option<&BinaryNode<T>> {
        self.left.as_deref()
    }

    pub fn right(&self) -> Option<&BinaryNode<T>> {
        self.right.as_deref()
    }

    pub fn append_left(&mut self, value: T) {
        if self.left.is_none() {
            self.left = Some(Box::new(BinaryNode::from_value(value)));
        }
    }

    pub fn append_right(&mut self, value: T) {
        if self.right.is_none() {
            self.right = Some(Box::new(BinaryNode::from_value(value)));
        }
    }
}

fn part_1(contents: &String) -> i32 {
    let mut above: Vec<char> = Vec::new();
    let mut count = 0;

    println!("{}", contents.lines().next().unwrap());
    for line in contents.lines() {
        if above.len() != line.len() {
            above.resize(line.len(), '\0');
        }
        for (i, char) in line.char_indices() {
            if char == 'S' {
                above[i] = '|';
                continue;
            } else if char == '^' {
                if above[i] == '|' || above[i] == 'S' {
                    count += 1;
                    if i >= 1 {
                        above[i - 1] = '|';
                    }
                    if i < line.len() - 1 {
                        above[i + 1] = '|';
                    }
                    above[i] = '^';
                    continue;
                }
            } else if char == '.' {
                if above[i] == '|' {
                    continue;
                }
            }
            above[i] = char;
        }
        for char in &above {
            print!("{char}");
        }
        println!("");
    }

    count
}

fn part_2(contents: &String) -> u64 {
    let root: BinaryNode<usize> = BinaryNode::new();
    let mut current_node: Box<BinaryNode<usize>>;
    // let mut layer: Vec<Rc<RefCell<BinaryNode<T>>>> = Vec::new();
    let mut above: Vec<u64> = Vec::new();
    let mut count = 0;
    for line in contents.lines() {
        if above.len() != line.len() {
            above.resize(line.len(), 0);
        }

        for (i, char) in line.char_indices() {
            if char == 'S' {
                above[i] = 1;
                continue;
            } else if char == '^' {
                if above[i] != 0 {
                    if i >= 1 {
                        above[i - 1] += above[i];
                    }
                    if i < line.len() - 1 {
                        above[i + 1] += above[i];
                    }
                    above[i] = 0;
                    continue;
                }
            } else if char == '.' {
                let a = 1;
            }
            // above[i] = char;
        }
    }
    for val in above {
        println!("Val {val}, count={count}");
        count += val;
    }
    count
}

fn main() {
    let filepath = "day_7/input/input.txt";
    let filepath = "day_7/input/test_input.txt";

    let contents = fs::read_to_string(filepath).expect("Something went wrong");
    let out_1 = part_1(&contents);
    let out_2 = part_2(&contents);

    println!("1 Final out: {out_1}");
    println!("2 Final out: {out_2}");
}
