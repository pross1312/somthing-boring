use std::fs::File;
use std::io::prelude::*;

fn main() {
    part1();
    part2();
}

fn error_value(c: u8) -> u32 {
    let mut result: u32 = 0;
    if (c as char).is_uppercase() { result += 26 }
    result += (((c as char).to_lowercase().nth(0).unwrap() as u8) - b'a') as u32;
    result + 1
}

fn part1() {
    let input_path = "input.txt";
    let mut input_file = File::open(input_path).unwrap_or_else(|_| panic!("Can't open file {}", input_path));
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap_or_else(|_| panic!("Can't read input from {}", input_path));
    let mut cache: [u8; 256] = [0 ; 256];
    let mut result = 0;
    for line in input.split_terminator('\n') {
        let first_part = line[ .. line.len()/2].as_bytes();
        let second_part: &[u8] = line[line.len()/2 .. ].as_bytes();
        for c in first_part {
            cache[*c as usize] += 1;
        }
        for c in second_part {
            if cache[*c as usize] != 0 {
                result += error_value(*c);
                break
            }
        }
        cache = [0 ; 256];
    }
    println!("Part1: {result}");
}

fn part2() {
    let input_path = "input.txt";
    let mut input_file = File::open(input_path).unwrap_or_else(|_| panic!("Can't open file {}", input_path));
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap_or_else(|_| panic!("Can't read input from {}", input_path));
    let mut cache: [u8; 256] = [0 ; 256];
    let mut result = 0;
    let mut common_char: u8 = 0;
    let mut count = 0;
    for line in input.split_terminator('\n') {
        for c in line.as_bytes() {
            if cache[*c as usize] == count { cache[*c as usize] += 1; }
            if cache[*c as usize] > cache[common_char as usize] {
                common_char = *c;
            }
        }
        count += 1;
        if count == 3 {
            result += error_value(common_char);
            common_char = 0;
            count = 0;
            cache = [0 ; 256];
        }
    }
    println!("Part1: {result}");
}
