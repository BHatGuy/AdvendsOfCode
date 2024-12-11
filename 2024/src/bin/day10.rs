use std::{collections::HashSet, fs};

fn read_map(name: &str) -> Vec<Vec<u32>> {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut map = Vec::new();
    for l in lines {
        let mut row = Vec::new();
        for c in l.chars() {
            let level = c.to_digit(10).expect("Could not parse level");
            row.push(level);
        }
        map.push(row);
    }

    map
}

fn score(row: usize, col: usize, map: &Vec<Vec<u32>>, scoring: bool) -> u32 {
    let mut score = 0;
    let mut fringe = vec![(row, col)];
    let mut visited = HashSet::new();
    while fringe.len() > 0 {
        let mut new_fringe = Vec::new();

        for (row, col) in fringe {
            let directions: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dc, dr) in directions {
                let next_row = row as i32 + dr;
                let next_col = col as i32 + dc;
                if next_row < 0
                    || next_row >= map.len() as i32
                    || next_col < 0
                    || next_col >= map[0].len() as i32
                {
                    continue;
                }
                let next_row = next_row as usize;
                let next_col = next_col as usize;
                if map[row][col] + 1 == map[next_row][next_col] {
                    if visited.contains(&(next_row, next_col)) && scoring {
                        continue;
                    }
                    visited.insert((next_row, next_col));
                    if map[next_row][next_col] == 9 {
                        score += 1;
                    } else {
                        new_fringe.push((next_row, next_col));
                    }
                }
            }
        }

        fringe = new_fringe;
    }
    return score;
}

fn main() {
    let map = read_map("input10.txt");
    let mut score_sum = 0;
    let mut rating_sum = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 0 {
                score_sum += score(row, col, &map, true);
                rating_sum += score(row, col, &map, false);
            }
        }
    }
    println!("{score_sum}");
    println!("{rating_sum}");
}
