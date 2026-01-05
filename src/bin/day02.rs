use aoc_2024::utils;
use std::process::exit;

fn part1_valid(report: &Vec<u64>) -> bool {
    if !report.is_sorted_by(|a, b| a < b) && !report.is_sorted_by(|a, b| a > b) {
        // println!("Invalid Report: {:?}", report);
        return false;
    }
    for window in report.windows(2) {
        let diff: u64 = (window[0] as i64 - window[1] as i64).abs() as u64;

        if diff == 0 || diff > 3 {
            // println!("Invalid Report: {:?}", report);
            return false;
        }
    }
    return true;
}

fn part1(reports: &Vec<Vec<u64>>) -> usize {
    let mut result: usize = 0;

    for report in reports {
        if part1_valid(report) {
            // println!("Report {report:?} is Valid");
            result += 1;
        }
    }

    return result;
}

fn part2(reports: &Vec<Vec<u64>>) -> usize {
    let mut result: usize = 0;
    let mut report_copy: Vec<u64> = Vec::new();

    for report in reports {
        // println!("=======================");
        // println!("testing report {report:?}");
        if part1_valid(report) {
            // println!("report {report:?} is valid");
            result += 1;
            continue;
        }

        let mut valid = false;
        for i in 0..report.len() {
            report_copy.clear();

            for j in 0..report.len() {
                if i != j {
                    report_copy.push(report[j]);
                }
            }
            // println!("report copy: {report_copy:?}");
            if part1_valid(&report_copy) {
                // println!("report copy {report_copy:?} is valid");
                valid = true;
                break;
            }
        }
        if valid {
            result += 1;
        }
    }

    return result;
}

fn main() {
    let content = utils::load_data().unwrap_or_else(|err| {
        eprintln!("[ERROR] Could not open input file: {}", err);
        exit(1);
    });
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
