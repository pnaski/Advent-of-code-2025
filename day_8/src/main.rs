use std::{ collections::HashSet, fs };

#[derive(Debug)]
struct FuseBox {
    x: u64,
    y: u64,
    z: u64,
}
impl FuseBox {
    pub fn new(x: u64, y: u64, z: u64) -> Self {
        FuseBox { x, y, z }
    }

    pub fn distance(&self, other: &FuseBox) -> u64 {
        self.x.abs_diff(other.x).pow(2) +
            self.y.abs_diff(other.y).pow(2) +
            self.z.abs_diff(other.z).pow(2)
    }
}

fn clean_chains<T>(v: &mut Vec<HashSet<T>>) {
    v.retain(|set| !set.is_empty());
}

fn largest_set_len<T>(v: &Vec<std::collections::HashSet<T>>) -> usize {
    v.iter()
        .map(|set| set.len())
        .max()
        .unwrap_or(0)
}

fn parse_file(contents: &String) -> Vec<FuseBox> {
    let mut ret: Vec<FuseBox> = Vec::new();
    for line in contents.lines() {
        let numbers: Vec<u64> = line
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        ret.push(FuseBox::new(numbers[0], numbers[1], numbers[2]));
    }

    ret
}

fn merge_chains(chains: &mut Vec<HashSet<usize>>) {
    let mut merged = true;
    while merged {
        merged = false;
        let mut join_chains = false;
        for i in 0..chains.len() {
            for j in 0..chains.len() {
                if i == j || chains[j].is_empty() || chains[i].is_empty() {
                    continue;
                }
                for element in chains[i].iter() {
                    if chains[j].contains(&element) {
                        merged = true;
                        join_chains = true;
                        break;
                    }
                }
                if join_chains {
                    let copy = chains[j].clone();
                    chains[i].extend(&copy);
                    chains[j].clear();
                    join_chains = false;
                }
            }
        }
    }
    // chains
}

fn part_1(contents: &String) -> u64 {
    let fuse_boxes = parse_file(contents);
    let mut chains: Vec<HashSet<usize>> = Vec::new();

    let mut distances: Vec<(usize, usize, u64)> = Vec::new();

    for i in 0..fuse_boxes.len() {
        for j in 0..fuse_boxes.len() {
            if i == j || j > i {
                continue;
            }
            distances.push((i, j, fuse_boxes[i].distance(&fuse_boxes[j])));
        }
    }
    distances.sort_by(|a, b| a.2.cmp(&b.2));
    distances.reverse();

    let mut counter = 0;
    loop {
        let (i, j, _) = match distances.pop() {
            Some(x) => x,
            None => {
                break;
            }
        };

        let mut last_chain = HashSet::new();
        last_chain.insert(i);
        last_chain.insert(j);
        chains.push(last_chain);

        counter += 1;
        if counter == 1000 {
            break;
        }
    }

    merge_chains(&mut chains);
    clean_chains(&mut chains);

    let mut lengths: Vec<usize> = Vec::new();

    for chain in chains {
        lengths.push(chain.len());
    }
    lengths.sort();
    lengths.reverse();
    (lengths[0] as u64) * (lengths[1] as u64) * (lengths[2] as u64)
}

fn part_2(contents: &String) -> u64 {
    let mut ret = 0;
    let fuse_boxes = parse_file(contents);
    let mut chains: Vec<HashSet<usize>> = Vec::new();
    let mut distances: Vec<(usize, usize, u64)> = Vec::new();

    for i in 0..fuse_boxes.len() {
        for j in 0..fuse_boxes.len() {
            if i == j || j > i {
                continue;
            }
            distances.push((i, j, fuse_boxes[i].distance(&fuse_boxes[j])));
        }
    }
    distances.sort_by(|a, b| a.2.cmp(&b.2));
    distances.reverse();
    println!("{}", distances.len());
    let mut counter = 0;
    let mut longest_chain = 0;
    loop {
        let (i, j, _) = match distances.pop() {
            Some(x) => x,
            None => {
                break;
            }
        };
        let mut last_chain = HashSet::new();
        last_chain.insert(i);
        last_chain.insert(j);
        chains.push(last_chain);
        if counter > 2000 {
            merge_chains(&mut chains);
            clean_chains(&mut chains);
            longest_chain = largest_set_len(&chains);

            println!("{counter} {longest_chain}");
            if longest_chain >= 1000 {
                println!("{:?} {:?}", fuse_boxes[i], fuse_boxes[j]);
                ret = fuse_boxes[i].x * fuse_boxes[j].x;
                break;
            }
        }
        counter += 1;
    }
    ret
}

fn main() {
    let filepath = "day_8/input/test_input.txt";
    let filepath = "day_8/input/input.txt";

    let contents = fs::read_to_string(filepath).expect("Something went wrong");
    let out_1 = part_1(&contents);
    let out_2 = part_2(&contents);

    println!("1 Final out: {out_1}");
    println!("2 Final out: {out_2}");
}
