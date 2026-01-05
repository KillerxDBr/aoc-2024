use aoc_2024::utils;
use regex::Regex;
use std::process::exit;

fn part1(content: &String) -> u64 {
    let mut result = 0;
    let mul_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    // for mat in re.find_iter(content) {
    for cap in mul_regex.captures_iter(content) {
        let a = cap[1].parse::<u64>().unwrap();
        let b = cap[2].parse::<u64>().unwrap();
        result += a * b;
    }

    return result;
}

fn part2(content: &String) -> u64 {
    let mut result = 0;

    // type DoRange = (usize, usize, bool);
    // let mut ranges: Vec<DoRange> = Vec::new();

    let mut ok = true;
    let mut i: usize = 0;
    let sz = content.len();
    while i < sz {
        let do_loc: usize;
        let dont_loc: usize;

        // println!("==============================");
        // println!("i:    {i}");
        if let Some(x) = content[i..].find(r"do()") {
            // i += x;
            do_loc = x;
        } else {
            do_loc = usize::MAX;
        }
        if let Some(x) = content[i..].find(r"don't()") {
            // i += x;
            dont_loc = x;
        } else {
            dont_loc = usize::MAX;
        }

        let str_end: usize;
        if ok {
            if dont_loc != usize::MAX {
                str_end = (i + dont_loc + 1).min(sz - 1);
            } else {
                str_end = sz;
            }
            let str = content[i..str_end].to_string();
            result += part1(&str);
            i = str_end;
            ok = false;
        } else {
            if do_loc != usize::MAX {
                str_end = (i + do_loc + 1).min(sz - 1);
            } else {
                str_end = sz;
            }
            i = str_end;
            ok = true
        }
    }
    return result;
}

fn main() {
    let content = utils::load_data().unwrap_or_else(|err| {
        eprintln!("[ERROR] Could not open input file: {}", err);
        exit(1);
    });
    println!("Resultado Parte 1: {}", part1(&content));
    println!("Resultado Parte 2: {}", part2(&content));
}
