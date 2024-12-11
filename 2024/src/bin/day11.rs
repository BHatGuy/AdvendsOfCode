use std::{collections::HashMap, fs, usize};

use num::Integer;

fn read_stones(name: &str) -> Vec<String> {
    let buf = fs::read_to_string(name).unwrap();
    buf.trim_end()
        .replace("\n", "")
        .split(" ")
        .map(|x| x.to_owned())
        .collect()
}

fn how_many_stones(
    stone: &str,
    n: usize,
    stone_map: &mut HashMap<(String, usize), usize>,
) -> usize {
    if n == 0 {
        return 1;
    }
    if let Some(c) = stone_map.get(&(stone.to_owned(), n)) {
        return *c;
    }

    let mut stones = Vec::new();
    if stone == "0" {
        stones.push("1".to_owned());
    } else if stone.len().is_even() {
        let stone_a = stone[0..stone.len() / 2].to_owned();
        let stone_b = &stone[stone.len() / 2..stone.len()];
        let stone_b = stone_b.parse::<u64>().unwrap().to_string();
        stones.push(stone_a);
        stones.push(stone_b);
    } else {
        let new_stone = (stone.parse::<u64>().unwrap() * 2024).to_string();
        stones.push(new_stone);
    }
    let mut sum = 0;
    for stone in stones {
        sum += how_many_stones(&stone, n - 1, stone_map);
    }

    stone_map.insert((stone.to_owned(), n), sum);
    sum
}

fn main() {
    let stones = read_stones("input11.txt");

    let mut sum = 0;
    let mut stone_map = HashMap::new();
    for stone in stones {
        sum += how_many_stones(&stone, 75, &mut stone_map);
    }
    println!("{}", sum);
}
