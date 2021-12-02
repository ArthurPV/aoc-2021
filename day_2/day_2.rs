use std::env::args;
use std::fs::read_to_string;

fn part_1(input: &Vec<String>) -> u32 {
    let mut h = 0;
    let mut d = 0;

    for (_, s) in input.iter().enumerate() {
        match s.chars().nth(0).unwrap() {
            'd' => {
                let n = s[5..].parse::<u32>().unwrap();
                d += n;
            }
            'f' => {
                let n = s[8..].parse::<u32>().unwrap();
                h += n;
            }
            'u' => {
                let n = s[3..].parse::<u32>().unwrap();
                d -= n;
            }
            _ => unreachable!(),
        }
    }

    h * d
}

fn part_2(input: &Vec<String>) -> u32 {
    let mut h = 0;
    let mut d = 0;
    let mut a = 0;

    for (_, s) in input.iter().enumerate() {
        match s.chars().nth(0).unwrap() {
            'd' => {
                let n = s[5..].parse::<u32>().unwrap();
                a += n;
            }
            'f' => {
                let n = s[8..].parse::<u32>().unwrap();
                h += n;
                d += n * a;
            }
            'u' => {
                let n = s[3..].parse::<u32>().unwrap();
                a -= n;
            }
            _ => unreachable!(),
        }
    }

    h * d
}

fn main() {
    let input = args().skip(1).next().expect("input file is not specified");
    let input_str = read_to_string(&input).unwrap();
    let v = input_str
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<String>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&v));
    println!("Part 2: {}", part_2(&v));
}
