use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut input_file = File::open("input.txt").expect("Can't open");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("can't read");
    let mut sum: i64 = 0;
    let mut ans: i64 = 0;
    let mut all: Vec<i64> = Vec::new();
    for token in input.split("\n") {
        if token != "" {
            let val: i64 = String::from(token).trim().parse().expect("Can't parse");
            sum += val;
        } else if sum > ans {
            ans = sum;
            all.push(sum);
            sum = 0;
        } else {
            all.push(sum);
            sum = 0;
        }
    }
    all.sort_by(|a, b| b.cmp(a));
    println!("Part2: {}", all[0] + all[1] + all[2]);
    println!("Part1: {}", all[0]);
}
