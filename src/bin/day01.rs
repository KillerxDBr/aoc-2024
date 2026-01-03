use aoc_2024::utils;

use std::cmp::max;
use std::iter;

fn part1(lhs: &Vec<u64>, rhs: &Vec<u64>) -> u64 {
    let mut lhs_sorted: Vec<u64> = lhs.clone();
    let mut rhs_sorted: Vec<u64> = rhs.clone();

    lhs_sorted.sort();
    rhs_sorted.sort();

    assert_eq!(lhs_sorted.len(), rhs_sorted.len());

    let mut result = 0;

    for i in 0..lhs_sorted.len() {
        let diff = lhs_sorted[i].abs_diff(rhs_sorted[i]);
        // println!("{}, {} = {}", &lhs[i], &rhs[i], diff);
        result += diff;
    }

    return result;
}

fn part2(lhs: &Vec<u64>, rhs: &Vec<u64>) -> u64 {
    let mut result: u64 = 0;

    // let r_max= ;
    // let l_max= ;
    let vec_max = *max(rhs.iter().max().unwrap(), lhs.iter().max().unwrap()) as usize + 1;
    // let mut rst: Vec<u8> = Vec::with_capacity(vec_max as usize);
    // println!("Reserving Vector with {} slots", vec_max);
    let mut rst: Vec<u64> = iter::repeat(0).take(vec_max).collect();

    for v in rhs {
        rst[*v as usize] += 1;
    }

    for i in lhs {
        if rst[*i as usize] == 0 {
            continue;
        }
        // println!("{} += {} * {}", result, i, rst[*i as usize]);
        result += *i * rst[*i as usize];
    }

    return result;
}

fn main() {
    println!("aeHOOOOO");
    let content = utils::load_data().unwrap();

    let mut lhs: Vec<u64> = Vec::new();
    let mut rhs: Vec<u64> = Vec::new();

    for line in content.lines() {
        // println!("Line: \"{}\"", line);
        let mut c = line.split_ascii_whitespace();
        lhs.push(c.next().unwrap().parse().unwrap());
        rhs.push(c.next().unwrap().parse().unwrap());

        assert_eq!(None, c.next());
    }

    println!("Resultado Parte 1: {}", part1(&lhs, &rhs));
    println!("Resultado Parte 2: {}", part2(&lhs, &rhs));
}
