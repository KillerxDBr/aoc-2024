use std::{env, str::FromStr};

fn load_data() -> Result<String, std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let path: String;
    if args.len() > 1 {
        path = args[1].clone();
    } else {
        path = String::from_str("input/day02/input.txt").unwrap();
    }

    return std::fs::read_to_string(path);
}

fn part1(reports: &Vec<Vec<u64>>) -> u64 {
    let mut result: u64 = 0;

    for report in reports {
        let mut safe: bool = true;
        let mut rc = report.clone();
        rc.sort();
        if rc != *report {
            rc.reverse();
            if rc != *report {
                // println!("Invalid Report: {:?}", report);
                continue;
            }
        }
        for i in 1..report.len() {
            let diff: u64 = (report[i - 1] as i64 - report[i] as i64).abs() as u64;

            if diff == 0 || diff > 3 {
                // println!("Invalid Report: {:?}", report);
                safe = false;
                break;
            }
        }
        if safe {
            // println!("Valid Report: {:?}", report);
            result += 1;
        }
    }

    return result;
}

fn part2(_reports: &Vec<Vec<u64>>) -> u64 {
    let mut _result: u64 = 0;

    //

    return _result;
}

fn main() {
    let content = load_data().unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let mut reports: Vec<Vec<u64>> = Vec::with_capacity(lines.len());
    for line in lines {
        let mut report: Vec<u64> = Vec::new();
        for v in line.split_whitespace() {
            report.push(v.parse::<u64>().unwrap());
        }
        reports.push(report);
    }

    println!("Resultado Parte 1: {}", part1(&reports));
    println!("Resultado Parte 2: {}", part2(&reports));
}
