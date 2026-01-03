use aoc_2024::utils;
use regex::Regex;

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

fn main() {
    let content = utils::load_data().unwrap();
    println!("Resultado Parte 1: {}", part1(&content));
}
