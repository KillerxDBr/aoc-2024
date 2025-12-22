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

fn part2(reports: &Vec<Vec<u64>>) -> u64 {
    let mut result: u64 = 0;

    for report in reports {
        let report_count = report.len();

        let mut safe: bool = true;
        let mut again: bool = false;
        let mut new_report = report.clone();

        println!("=================================================");
        println!("Testing Report: {report:?}");
        let ord = new_report[0] as i64 - new_report[1] as i64;
        for i in 1..report_count {
            // diffs.push(report[i] as i64 - report[i - 1] as i64);
            let diff: i64 = new_report[i - 1] as i64 - new_report[i] as i64;
            println!(
                "abs({} - {}) = {}",
                new_report[i - 1],
                new_report[i],
                diff.abs()
            );
            if diff.abs() == 0 || diff.abs() > 3 || (ord < 0 && diff > 0) || (ord > 0 && diff < 0) {
                println!("ord: {ord}");
                println!("diff: {diff}");
                println!("diff.abs() == 0: {}", diff.abs() == 0);
                println!("diff.abs() > 3: {}", diff.abs() > 3);
                println!("ord < 0 && diff > 0: {}", (ord < 0 && diff > 0));
                println!("ord > 0 && diff < 0: {}", (ord > 0 && diff < 0));

                let index_to_remove: usize;
                if diff.abs() == 0 || ord < 0 {
                    index_to_remove = i - 1;
                } else {
                    index_to_remove = i;
                }
                println!(
                    "Removing report {} ({})",
                    index_to_remove, new_report[index_to_remove]
                );
                new_report.remove(index_to_remove);
                again = true;
                break;
            }
        }

        if again {
            println!("Testing Again");
            let ord = new_report[0] as i64 - new_report[1] as i64;
            for i in 1..new_report.len() {
                let diff: i64 = new_report[i - 1] as i64 - new_report[i] as i64;
                let u_diff = diff.abs() as u64;
                println!("abs({} - {}) = {u_diff}", new_report[i - 1], new_report[i]);
                if u_diff == 0 || u_diff > 3 || (ord < 0 && diff > 0) || (ord > 0 && diff < 0) {
                    println!("diff.abs() == 0: {}", diff.abs() == 0);
                    println!("diff.abs() > 3: {}", diff.abs() > 3);
                    println!("ord < 0 && diff > 0: {}", (ord < 0 && diff > 0));
                    println!("ord > 0 && diff < 0: {}", (ord > 0 && diff < 0));
                    println!("Report {report:?} is NOT safe");
                    safe = false;
                    break;
                }
            }
        }
        if safe {
            println!("Report {report:?} is safe");
            result += 1;
        }
    }
    return result;
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
