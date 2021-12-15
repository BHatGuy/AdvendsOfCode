use std::fs;
use std::collections::HashMap;

type InputType = Vec<Vec<u64>>;

fn get_input(name: &str) -> InputType {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut input = Vec::new();

    for l in lines {
        let row = l.trim().chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
        input.push(row);
    }
    input
}



fn solve1(matrix: &InputType) -> u64 {
    let mut paths = HashMap::new();
    let end = (matrix[0].len(), matrix.len());
    paths.insert((0, 0), 0);

    while !paths.contains_key(&end) {

    }
    *paths.get(&end).unwrap()
}

fn solve2(matrix: &InputType) -> u64 {
    0
}

fn main() {
    let input = get_input("test.txt");
    println!("1: {}", solve1(&input));
    println!("2: {}", solve2(&input));
}
