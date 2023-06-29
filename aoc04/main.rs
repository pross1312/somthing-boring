fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let data = line.split_terminator(',')
                       .map(|x| x.split('-')
                                 .map(|x| x.parse::<i32>().unwrap())
                                 .collect::<Vec<i32>>())
                       .collect::<Vec<Vec<i32>>>();
        if data[0][0] <= data[1][0] && data[0][1] >= data[1][1] ||
           data[1][0] <= data[0][0] && data[1][1] >= data[0][1] {
            part1 += 1;
            part2 += 1;
        } else if (data[0][1] >= data[1][0] && data[0][1] <= data[1][1]) ||
                  (data[0][0] >= data[1][0] && data[0][0] <= data[1][1]) {
            part2 += 1;
        }
    }
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
