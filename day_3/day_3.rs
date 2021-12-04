use std::env::args;
use std::fs::read_to_string;

fn part_1(input: &Vec<String>) -> usize {
    let mut gama = String::from("");
    let mut espion = String::from("");

    for i in 0..12 {
        let mut n_0 = 0;
        for (_, v) in input.iter().enumerate() {
            if v.chars().nth(i).unwrap() == '0' {
                n_0 += 1;
            }
        }
        if n_0 > input.len() / 2 {
            gama.push('0');
            espion.push('1');
        } else {
            gama.push('1');
            espion.push('0');
        }
    }

    let n1 = usize::from_str_radix(gama.as_str(), 2).unwrap();
    let n2 = usize::from_str_radix(espion.as_str(), 2).unwrap();

    n1 * n2
}

fn keep_value(idx_v: &Vec<usize>, s_v: &mut Vec<String>) {
    let mut k_v = vec![];
    for (_, v) in idx_v.iter().enumerate() {
        k_v.push(s_v[*v].to_string());
    }
    *s_v = k_v;
}

fn part_2(input: &Vec<String>) -> usize {
    let mut gen = input.clone();
    let mut co2 = input.clone();

    for i in 0..12 {
        let mut n_0 = vec![];
        let mut n_1 = vec![];

        if gen.len() > 1 {
            for (n, s) in gen.iter().enumerate() {
                if s.chars().nth(i).unwrap() == '0' {
                    n_0.push(n);
                } else {
                    n_1.push(n);
                }
            }

            if n_0.len() > n_1.len() {
                keep_value(&n_0, &mut gen);
            } else if n_0.len() <= n_1.len() {
                keep_value(&n_1, &mut gen);
            }
        }

        n_0 = vec![];
        n_1 = vec![];

        if co2.len() > 1 { 
            for (n, s) in co2.iter().enumerate() {
                if s.chars().nth(i).unwrap() == '0' {
                    n_0.push(n);
                } else {
                    n_1.push(n);
                }
            }

            if n_0.len() <= n_1.len() {
                keep_value(&n_0, &mut co2);
            } else if n_0.len() > n_1.len() {
                keep_value(&n_1, &mut co2);
            }
        }
    }

    let n1 = usize::from_str_radix(gen[0].as_str(), 2).unwrap();
    let n2 = usize::from_str_radix(co2[0].as_str(), 2).unwrap();

    n1 * n2
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
