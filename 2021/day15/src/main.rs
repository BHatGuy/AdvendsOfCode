use std::collections::{HashMap, HashSet};
use std::fs;

type InputType = Vec<Vec<u64>>;

const OFFSETS: &[(i32, i32)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

fn get_input(name: &str) -> InputType {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut input = Vec::new();

    for l in lines {
        let row = l
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        input.push(row);
    }
    input
}

fn find_path(matrix: &InputType) -> u64 {
    let mut paths = HashMap::new();
    let mut visited = HashSet::new();
    let end = (matrix[0].len() - 1, matrix.len() - 1);
    paths.insert((0, 0), 0);
    while !paths.contains_key(&end) {
        let min = paths
            .iter()
            .reduce(|a, b| std::cmp::min_by_key(a, b, |x| x.1))
            .unwrap();
        let min = (*min.0, *min.1);
        for off in OFFSETS {
            let offx = (min.0 .0 as i32 + off.0) as usize;
            let offy = (min.0 .1 as i32 + off.1) as usize;
            if offy >= matrix.len() || offx >= matrix[0].len() {
                continue;
            }
            if visited.contains(&(offx, offy)) {
                continue;
            }
            let new_risk = matrix[offy][offx] + min.1;
            let risk = paths.entry((offx, offy)).or_insert(u64::MAX);
            if new_risk < *risk {
                *risk = new_risk;
            }
        }
        paths.remove(&min.0);
        visited.insert(min.0);
    }
    *paths.get(&end).unwrap()
}

fn solve1(matrix: &InputType) -> u64 {
    find_path(matrix)
}

fn add(matrix: &mut InputType, a: u64) {
    for x in matrix.iter_mut().flatten() {
        *x += a;
        if *x > 9 {
            *x -= 9;
        }
    }
}

fn solve2(matrix: &InputType) -> u64 {
    let mut larger = matrix.clone();
    for i in 1..5 {
        let mut tile = matrix.clone();
        add(&mut tile, i);
        for (row, extra) in larger.iter_mut().zip(tile.iter_mut()) {
            row.append(extra);
        }
    }
    let top = larger.clone();
    for i in 1..5 {
        let mut tile = top.clone();
        add(&mut tile, i);
        larger.append(&mut tile);
    }
    find_path(&larger)
}

fn main() {
    let input = get_input("input.txt");
    println!("1: {}", solve1(&input));
    println!("2: {}", solve2(&input));
}
