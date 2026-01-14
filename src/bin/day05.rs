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

fn check_order(rules: &HashMap<usize, Vec<usize>>, order: &[usize]) -> bool {
    for i in 1..order.len() {
        let k = &order[i];
        let slice = &order[..i];
        if let Some(r) = rules.get(k) {
            for n in r {
                if slice.contains(n) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn reorder(rules: &HashMap<usize, Vec<usize>>, order: &[usize]) -> usize {
    let mut reorderer = order.to_vec();

    let mut index: usize = 1;
    while index < reorderer.len() {
        let k = reorderer[index];
        let slice = &reorderer[..index];

        if let Some(r) = rules.get(&k) {
            for n in r {
                if slice.contains(n) {
                    let j: usize = slice.iter().position(|&x| x == *n).unwrap();

                    println!("========================");
                    println!("antes:  {:?}", reorderer);
                    reorderer.swap(index, j);
                    println!("depois: {:?}", reorderer);

                    break;
                }
            }
        }

        let t = reorderer[..=index].to_vec();
        println!("Testing {:?}", &t);
        if check_order(rules, &t) {
            index += 1;
        }
    }

    return reorderer[reorderer.len() / 2];
}

fn process(rules: &HashMap<usize, Vec<usize>>, orders: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut result1: usize = 0;
    let mut result2: usize = 0;

    for order in orders {
        if check_order(rules, order) {
            println!("Order {order:?} is valid");
            result1 += order[order.len() / 2];
        // } else {
        //     println!("Order {order:?} is invalid, reordering...");
        //     result2 += reorder(rules, order);
        }
    }

    return (result1, result2);
}

fn main() {
    let content = utils::load_data().unwrap_or_else(|err| {
        eprintln!("[ERROR] Could not open input file: {}", err);
        exit(1);
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

    let (result1, result2) = process(&rules, &orders);

    println!("Resultado Parte 1: {}", result1);
    println!("Resultado Parte 2: {}", result2);
}
