use aoc_2024::utils;
use std::{cmp::Ordering, process::exit};

fn search1(map: &Vec<&[u8]>, x: usize, y: usize) -> usize {
    let mut result: usize = 0;
    let word = [b'X', b'M', b'A', b'S'];

    let up = y >= 3;
    let dw = y < map.len() - 3;
    let lf = x >= 3;
    let rt = x < map[y].len() - 3;

    let mut pos: &[u8];
    // XMAS
    if rt {
        pos = &map[y][x..(x + 4)];
        if word.iter().cmp(pos.iter()) == Ordering::Equal {
            result += 1;
        }
    }

    // SAMX
    if lf {
        pos = &map[y][(x - 3)..(x + 1)];
        if word.iter().rev().cmp(pos.iter()) == Ordering::Equal {
            result += 1;
        }
    }

    // X
    // M
    // A
    // S
    if dw {
        let mut valid = true;
        for i in 0..4 {
            if word[i] != map[y + i][x] {
                valid = false;
                break;
            }
        }
        if valid {
            result += 1;
        }
    }

    // S
    // A
    // M
    // X
    if up {
        let mut valid = true;
        for i in 0..4 {
            if word[i] != map[y - i][x] {
                valid = false;
                break;
            }
        }
        if valid {
            result += 1;
        }
    }

    // X
    //  M
    //   A
    //    S
    if rt && dw {
        let mut valid = true;
        for i in 0..4 {
            if word[i] != map[y + i][x + i] {
                valid = false;
                break;
            }
        }
        if valid {
            result += 1;
        }
    }

    // S
    //  A
    //   M
    //    X
    if lf && up {
        let mut valid = true;
        for i in 0..4 {
            if word[i] != map[y - i][x - i] {
                valid = false;
                break;
            }
        }
        if valid {
            result += 1;
        }
    }

    //    S
    //   A
    //  M
    // X
    if rt && up {
        let mut valid = true;
        for i in 0..4 {
            if word[i] != map[y - i][x + i] {
                valid = false;
                break;
            }
        }
        if valid {
            result += 1;
        }
    }

    //    X
    //   M
    //  A
    // S
    if lf && dw {
        let mut valid = true;
        for i in 0..4 {
            if word[i] != map[y + i][x - i] {
                valid = false;
                break;
            }
        }
        if valid {
            result += 1;
        }
    }
    assert!(result <= 6, "result = {result}");
    return result;
}

fn search2(map: &Vec<&[u8]>, x: usize, y: usize) -> bool {
    let check_char = |c: u8| -> bool { return c == b'M' || c == b'S' };

    if y == 0 || y == map.len() - 1 {
        return false;
    }
    if x == 0 || x == map[y].len() - 1 {
        return false;
    }

    let tl = map[y - 1][x - 1];
    let tr = map[y - 1][x + 1];
    let bl = map[y + 1][x - 1];
    let br = map[y + 1][x + 1];

    if !check_char(tl) || !check_char(tr) || !check_char(bl) || !check_char(br) {
        return false;
    }

    if tl == br || tr == bl {
        return false;
    }

    return true;
}

fn part1(content: &String) -> usize {
    let mut result: usize = 0;

    let mut map: Vec<&[u8]> = Vec::new();
    for l in content.lines() {
        map.push(l.as_bytes());
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == b'X' {
                result += search1(&map, x, y);
            }
        }
    }

    return result;
}

fn part2(content: &String) -> usize {
    let mut result: usize = 0;

    let mut map: Vec<&[u8]> = Vec::new();
    for l in content.lines() {
        map.push(l.as_bytes());
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == b'A' {
                if search2(&map, x, y) {
                    result += 1;
                }
            }
        }
    }

    return result;
}

fn main() {
    let content = utils::load_data().unwrap_or_else(|err| {
        eprintln!("[ERROR] Could not open input file: {}", err);
        exit(err.raw_os_error().unwrap());
    });
    // println!("{content}");
    println!("Resultado Parte 1: {}", part1(&content));
    println!("Resultado Parte 2: {}", part2(&content));
}
