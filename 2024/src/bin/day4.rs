use std::{collections::HashMap, fs};

use ndarray::{s, Array2};

fn read_puzzle(name: &str) -> Vec<Vec<char>> {
    let buf = fs::read_to_string(name).expect("Cant read input");
    buf.trim_end()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect()
}

type Direction = (i32, i32);
const DIRECTIONS: [Direction; 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    let puzzle = read_puzzle("input4.txt");
    let mut count = 0;
    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            if puzzle[y][x] != 'X' {
                continue;
            }
            let mut candidates = HashMap::new();
            let mut results = HashMap::new();
            for d in DIRECTIONS {
                candidates.insert(d, (i32::try_from(x).unwrap(), i32::try_from(y).unwrap()));
                results.insert(d, "".to_string());
            }
            for _ in 0..4 {
                let mut next_candidates = HashMap::new();
                for d in candidates.keys() {
                    let pos = candidates.get(d).unwrap();
                    let c =
                        puzzle[usize::try_from(pos.1).unwrap()][usize::try_from(pos.0).unwrap()];
                    results.get_mut(d).unwrap().push(c);
                    let next = (pos.0 + d.0, pos.1 + d.1);
                    if next.0 < 0
                        || next.1 < 0
                        || (next.0 as usize) >= puzzle[0].len()
                        || (next.1 as usize) >= puzzle.len()
                    {
                        continue;
                    }
                    next_candidates.insert(*d, next);
                }
                candidates = next_candidates;
            }
            for word in results.values() {
                let word: String = word.to_string();
                if word == "XMAS" {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");

    let mut count = 0;
    let mut matrix = Array2::<char>::from_elem((puzzle[0].len(), puzzle.len()), ' ');
    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            matrix[[y, x]] = puzzle[y][x];
        }
    }
    for row in 0..matrix.shape()[0] - 2 {
        for col in 0..matrix.shape()[1] - 2 {
            let slice = matrix.slice(s![row..row + 3, col..col + 3]);
            let mut corners = String::new();
            corners.push(slice[[0, 0]]);
            corners.push(slice[[0, 2]]);
            corners.push(slice[[2, 0]]);
            corners.push(slice[[2, 2]]);

            let ms = corners.matches("S").count();
            let ss = corners.matches("M").count();
            if ms != 2 || ss != 2 {
                continue;
            }

            if slice[[1, 1]] == 'A' {
                if (slice[[0, 0]] == 'M' && slice[[2, 2]] == 'M')
                    || (slice[[0, 0]] == 'S' && slice[[2, 2]] == 'S')
                {
                    continue;
                }

                count += 1;
            }
        }
    }
    println!("{count}");
}
