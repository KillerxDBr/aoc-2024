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
        let mut diff: i64;
        let mut old_diff: i64 = 0;
        let mut old_diff_used: bool = false;

        let mut report_copy = report.clone();
        for i in 1..report.len() {
            diff = report[i] as i64 - report[i - 1] as i64;
            if diff == 0 && !old_diff_used {
                report_copy.remove(i - 1);
                break;
            }
            if diff > 3 || diff < -3 {
                report_copy.remove(i);
                break;
            }
            if old_diff_used {
                if diff == 0 || (diff > 0 && old_diff < 0) {
                    report_copy.remove(i - 1);
                    break;
                } else if (diff < 0 && old_diff > 0)
                    || (old_diff > 0 && diff > 3)
                    || (old_diff < 0 && diff < -3)
                {
                    report_copy.remove(i);
                    break;
                }
            }
            old_diff = diff;
            old_diff_used = true;
        }

        let mut safe = true;
        println!("Report:      {report:?}");

        if report.as_ref() != report_copy {
            old_diff_used = false;
            println!("Report Copy: {report_copy:?}");

            for i in 1..report_copy.len() {
                diff = report_copy[i] as i64 - report_copy[i - 1] as i64;
                if diff == 0 || diff > 3 || diff < -3 {
                    safe = false;
                    break;
                }
                if old_diff_used {
                    if diff > 0 && old_diff < 0 {
                        safe = false;
                        break;
                    } else if diff < 0 && old_diff > 0 {
                        safe = false;
                        break;
                    }
                }
                old_diff = diff;
                old_diff_used = true;
            }
        }

        if safe {
            println!("Report is Safe");
            result += 1;
        } else {
            println!("Report is NOT Safe");
        }

        // println!("Report: {report:?} dir: {}", if dir > 0 {"Ascending"} else {"Descending"});
        // if dir == 0 {
        //     println!("Report: {report:?} is invalid");
        //     continue;
        // }
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
