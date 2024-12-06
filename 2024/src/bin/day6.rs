use std::{collections::HashMap, fs};
fn read_map(name: &str) -> Vec<Vec<char>> {
    let buf = fs::read_to_string(name).expect("Cant read input");
    buf.trim_end()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect()
}

fn loops(map: &Vec<Vec<char>>, start_pos: (i32, i32)) -> bool {
    let mut visited: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let mut pos = start_pos;
    let mut direction = (-1, 0);
    let height = i32::try_from(map.len()).unwrap();
    let width = i32::try_from(map[0].len()).unwrap();
    visited.insert(pos, vec![direction]);
    loop {
        let new_pos = (pos.0 + direction.0, pos.1 + direction.1);
        if new_pos.0 < 0 || new_pos.0 >= height || new_pos.1 < 0 || new_pos.1 >= width {
            return false;
        }
        if map[usize::try_from(new_pos.0).unwrap()][usize::try_from(new_pos.1).unwrap()] == '#' {
            let tmp = direction.0;
            direction.0 = direction.1;
            direction.1 = tmp;
            if direction.0 == 0 {
                direction.1 *= -1;
            }
            continue;
        }
        pos = new_pos;
        if let Some(entry) = visited.get_mut(&pos) {
            if entry.contains(&direction) {
                return true;
            } else {
                entry.push(direction);
            }
        } else {
            visited.insert(pos, vec![direction]);
        }
    }
}

fn main() {
    let original_map = read_map("input6.txt");
    let mut map = original_map.clone();
    let height = map.len();
    let width = map[0].len();

    let mut start_pos = (0, 0);
    for row in 0..height {
        for col in 0..width {
            if map[row][col] == '^' {
                map[row][col] = 'X';
                start_pos = (i32::try_from(row).unwrap(), i32::try_from(col).unwrap());
            }
        }
    }

    let height = i32::try_from(map.len()).unwrap();
    let width = i32::try_from(map[0].len()).unwrap();
    let mut pos = start_pos;
    let mut direction = (-1, 0);
    loop {
        let new_pos = (pos.0 + direction.0, pos.1 + direction.1);
        if new_pos.0 < 0 || new_pos.0 >= height || new_pos.1 < 0 || new_pos.1 >= width {
            break;
        }
        if map[usize::try_from(new_pos.0).unwrap()][usize::try_from(new_pos.1).unwrap()] == '#' {
            let tmp = direction.0;
            direction.0 = direction.1;
            direction.1 = tmp;
            if direction.0 == 0 {
                direction.1 *= -1;
            }
            continue;
        }
        map[usize::try_from(new_pos.0).unwrap()][usize::try_from(new_pos.1).unwrap()] = 'X';
        pos = new_pos;
    }
    let mut count = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'X' {
                count += 1;
            }
        }
    }
    println!("{count}");

    let mut loop_count = 0;
    let mut map = original_map.clone();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '.' {
                map[row][col] = '#';
                if loops(&map, start_pos) {
                    loop_count += 1;
                }
                map[row][col] = '.';
            }
        }
    }
    println!("{loop_count}");
}
