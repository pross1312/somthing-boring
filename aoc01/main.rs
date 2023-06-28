fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sum: i64 = 0;
    let mut ans: i64 = 0;
    let mut all: Vec<i64> = Vec::new();
    for token in input.split_terminator('\n') {
        if token != "" {
            let val: i64 = token.trim().parse().unwrap();
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
