use std::collections::VecDeque;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    const N_STACKS: usize = 9;
    let mut stacks_1: [VecDeque<&str>; N_STACKS] = std::array::from_fn(|_| VecDeque::<&str>::new());
    let mut stacks_2: [Vec<&str>; N_STACKS] = std::array::from_fn(|_| Vec::<&str>::new());
    let mut switch = false;
    for line in input.lines() {
        let mut end = 0;
        if line.trim() == "" {
            switch = true;
            continue;
        }
        if !switch {
            while let Some(index) = line[end..].find('[') {
                let start = index + end;
                end = start + line[start..].find(']').unwrap();
                stacks_1[start / 4].push_front(&line[start..end+1]);
                stacks_2[start / 4].insert(0, &line[start..end+1]);
            }
        } else {
            let tokens: Vec<&str> = line.split_terminator(' ').collect();
            let n_items = tokens[1].parse::<usize>().unwrap();
            let from = tokens[3].parse::<usize>().unwrap() - 1;
            let to = tokens[5].parse::<usize>().unwrap() - 1;
            let stack_2_end: usize = stacks_2[to].len();
            for _ in 0..n_items {
                if let Some(val) = stacks_1[from].pop_back() {
                    stacks_1[to].push_back(val);
                }
                if let Some(val) = stacks_2[from].pop() {
                    stacks_2[to].insert(stack_2_end, val);
                }
            }
        }
    }
    print!("Part 1: ");
    for stack in stacks_1 {
        print!("{}", &stack.back().unwrap()[1..2]);
    }
    println!();
    print!("Part 2: ");
    for stack in stacks_2 {
        print!("{}", &stack.last().unwrap()[1..2]);
    }
    println!();
}
