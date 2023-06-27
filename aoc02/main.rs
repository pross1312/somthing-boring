use std::fs::File;
use std::io::prelude::*;
fn solve() {
    let mut input_file = File::open("input.txt").expect("Can't open file");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("Can't read input");
    let mut part1 = 0;
    let mut part2 = 0;
    for game in input.split('\n') {
        if game == "" {
            continue;
        }
        let a: Vec<&str> = game.split(' ').collect();
        let you = a[1].as_bytes()[0] - b'X';
        let oppo = a[0].as_bytes()[0] - b'A';
        part1 += match_point(you, oppo);
        part2 += match_point2(you, oppo);
    }
    println!("Part1 answer: {part1}");
    println!("Part2 answer: {part2}");
}
// 1 2 3
// r p s
fn match_point(mut you: u8, mut oppo: u8) -> i32 {
    let outcome = if you == oppo {3} else if (you+1)%3 == oppo {0} else {6};
    (you + 1 + outcome).into()
}

fn match_point2(mut you: u8, oppo: u8) -> i32 {
    if you == 1 {
        you = oppo;
    } else if you == 2 {
        you = if oppo == 2 { 0 } else {oppo + 1};
    } else {
        you = if oppo == 0 { 2 } else {oppo - 1};
    }
    match_point(you, oppo)
}

fn main() {
    solve();
}
