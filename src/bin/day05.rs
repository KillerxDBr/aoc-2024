use aoc_2024::utils;
use std::{collections::HashMap, process::exit};

fn parse_data(content: &String) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();

    for l in content.lines() {
        if l.len() == 0 {
            break;
        }

        let sep = l.find('|').unwrap();

        let index: usize = l[..sep].parse().unwrap();
        let element: usize = l[sep + 1..].parse().unwrap();

        rules.entry(index).or_insert_with(Vec::new).push(element);
    }

    let mut orders: Vec<Vec<usize>> = Vec::new();
    for l in content.lines() {
        if l.len() == 0 || l.contains('|') {
            continue;
        }

        let mut v: Vec<usize> = Vec::new();
        for n in l.split(',') {
            v.push(n.parse().unwrap());
        }
        orders.push(v);

        // println!("{l}");
    }

    return (rules, orders);
}

fn part1(rules: &HashMap<usize, Vec<usize>>, orders: &Vec<Vec<usize>>) -> usize {
    let mut result: usize = 0;

    for order in orders {
        let mut valid = true;
        'order_iter: for i in 1..order.len() {
            let k = &order[i];
            let slice = &order[..i];
            if let Some(r) = rules.get(k) {
                for n in r {
                    if slice.contains(n) {
                        valid = false;
                        break 'order_iter;
                    }
                }
            }
        }
        if valid {
            println!("Order {order:?} is valid");
            result += order[order.len() / 2];
        }
    }

    return result;
}

// fn part2(rules: &HashMap<usize, Vec<usize>>, order: &Vec<Vec<usize>>) -> usize {
//     let result: usize = 0;

//     // println!("{}", content);

//     return result;
// }

fn main() {
    let content = utils::load_data().unwrap_or_else(|err| {
        eprintln!("[ERROR] Could not open input file: {}", err);
        exit(err.raw_os_error().unwrap());
    });

    let (rules, orders) = parse_data(&content);

    println!("Rules:");
    for (k, v) in &rules {
        println!("  {k}: {v:?}");
    }
    println!("Orders:");
    for o in &orders {
        println!("  {o:?}");
    }

    println!("Resultado Parte 1: {}", part1(&rules, &orders));
    // println!("Resultado Parte 2: {}", part2(&rules, &orders));
}
