use std::env::args;
use std::fs::read_to_string;

fn part_1(input: &Vec<u32>) -> u32 {
    let mut n = 0;
    for (i, v) in input.iter().enumerate() {
        if i < input.len() - 1 {
            if v < &input[i + 1] {
                n += 1;
            }
        }
    }
    n
}

fn part_2(input: &Vec<u32>) -> u32 {
    let mut n = 0;
    for (i, _) in input.iter().enumerate() {
        if i < input.len() - 3 {
            let sum1 = input[i] + input[i + 1] + input[i + 2];
            let sum2 = input[i + 1] + input[i + 2] + input[i + 3];
            if sum1 < sum2 {
                n += 1;
            }
        }
    }
    n
}

fn main() {
    let input = args().skip(1).next().expect("input file is not specified");
    let input_str = read_to_string(&input).unwrap();
    let v = input_str
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&v));
    println!("Part 2: {}", part_2(&v));
}
