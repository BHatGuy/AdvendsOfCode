use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use ndarray::{arr2, Array2};

fn read_antennas(name: &str) -> Vec<Vec<Array2<i32>>> {
    let buf = fs::read_to_string(name).expect("Cant read input");
    let lines = buf.trim_end().split("\n");
    let mut map = HashMap::new();
    for (row, line) in lines.enumerate() {
        for (col, antenna) in line.chars().enumerate() {
            if antenna == '.' {
                continue;
            }
            let pos = arr2(&[[i32::try_from(row).unwrap(), i32::try_from(col).unwrap()]]);
            map.entry(antenna)
                .and_modify(|list: &mut Vec<Array2<i32>>| list.push(pos.clone()))
                .or_insert(vec![pos]);
        }
    }
    let mut groups = Vec::new();
    for g in map.values() {
        groups.push(g.clone());
    }
    groups
}

fn main() {
    let map = read_antennas("input8.txt");
    let mut antennas = HashSet::new();
    for group in &map {
        for combo in group.iter().combinations(2) {
            let a = combo[0];
            let b = combo[1];
            let antenna1 = 2 * a - b;
            let antenna2 = 2 * b - a;
            if antenna1.iter().all(|&x| x >= 0 && x < 50) {
                antennas.insert(antenna1);
            }
            if antenna2.iter().all(|&x| x >= 0 && x < 50) {
                antennas.insert(antenna2);
            }
        }
    }
    println!("{}", antennas.len());

    let mut antennas = HashSet::new();
    for group in map {
        for combo in group.iter().combinations(2) {
            let a = combo[0];
            let b = combo[1];
            let mut delta = a - b;
            let gcd = num::integer::gcd(delta[[0, 0]], delta[[0, 1]]);
            delta = delta.map(|x| x / gcd);
            let mut pos = a.clone();
            loop {
                antennas.insert(pos.clone());
                pos = pos - delta.clone();
                if pos.iter().any(|&x| x < 0 || x >= 50) {
                    break;
                }
            }
            let mut pos = a.clone();
            loop {
                antennas.insert(pos.clone());
                pos = pos + delta.clone();
                if pos.iter().any(|&x| x < 0 || x >= 50) {
                    break;
                }
            }
        }
    }
    println!("{}", antennas.len());
}
