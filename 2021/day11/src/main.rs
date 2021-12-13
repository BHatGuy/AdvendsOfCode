use std::collections::HashSet;
use std::fs;

type InputType = Vec<Vec<u32>>;
const OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

fn get_input(name: &str) -> InputType {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut input = Vec::new();

    for l in lines {
        let row = l.trim().chars().map(|x| x.to_digit(10).unwrap()).collect();
        input.push(row);
    }
    input
}


fn step(matrix: &mut InputType) {
    let mut to_flash = Vec::new();
    let mut flashed = HashSet::new();
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            matrix[y][x] += 1;
            if matrix[y][x] > 9 {
                to_flash.push((x, y));
                flashed.insert((x, y));
            }
        }
    }
    while to_flash.len() > 0 {
        let pos = to_flash.pop().unwrap();

        for off in OFFSETS {
            let offx = (pos.0 as i32 + off.0) as usize;
            let offy = (pos.1 as i32 + off.1) as usize;
            if offy >= matrix.len() || offx >= matrix[pos.1].len() {
                continue;
            }
            matrix[offy][offx] += 1;
            if matrix[offy][offx] > 9 && !flashed.contains(&(offx, offy)) {
                to_flash.push((offx, offy));
                flashed.insert((offx, offy));
            }
        }
    }
    for pos in flashed {
        matrix[pos.1][pos.0] = 0;
    }
}

fn solve1(mut matrix: InputType) -> usize {
    let mut flash_count = 0;
    for _ in 0..100 {
        step(&mut matrix);
        flash_count += matrix.iter().flatten().filter(|&&x| x == 0).count();
    }
    flash_count
}

fn solve2(mut matrix: InputType) -> u64 {
    let mut count = 0;
    loop {
        count += 1;
        step(&mut matrix);
        if matrix.iter().flatten().all(|&x| x == 0) {
            return count;
        }
    }
}

fn main() {
    let input = get_input("input.txt");
    println!("1: {}", solve1(input.clone()));
    println!("2: {}", solve2(input.clone()));
}
