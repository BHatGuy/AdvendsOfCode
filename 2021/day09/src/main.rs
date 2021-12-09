use std::collections::HashSet;
use std::fs;
use termion::color;

const OFFSETS: &[(i32, i32)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

fn get_input(name: &str) -> Vec<Vec<u32>> {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut input = Vec::new();

    for l in lines {
        let mut row = Vec::new();
        for c in l.chars() {
            row.push(c.to_digit(10).unwrap())
        }
        input.push(row);
    }
    input
}
fn get_lowpoints(matrix: &Vec<Vec<u32>>) -> HashSet<(usize, usize)> {
    let mut lowpoints = HashSet::new();
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let mut lowest = true;
            for off in OFFSETS {
                let offx = (x as i32 + off.0) as usize;
                let offy = (y as i32 + off.1) as usize;
                if offy >= matrix.len() || offx >= matrix[y].len() {
                    continue;
                }
                lowest &= matrix[y][x] < matrix[offy][offx];
            }
            if lowest {
                lowpoints.insert((x, y));
            }
        }
    }
    lowpoints
}

fn get_basins(matrix: &Vec<Vec<u32>>) -> Vec<HashSet<(usize, usize)>> {
    let lowpoints = get_lowpoints(matrix);
    let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();
    for lp in lowpoints {
        let mut basin = HashSet::new();
        let mut to_visit = Vec::new();
        to_visit.push(lp);
        while to_visit.len() > 0 {
            let p = to_visit.pop().unwrap();
            for off in OFFSETS {
                let offx = (p.0 as i32 + off.0) as usize;
                let offy = (p.1 as i32 + off.1) as usize;
                if offy >= matrix.len() || offx >= matrix[p.1].len() {
                    continue;
                }
                if matrix[offy][offx] != 9
                    && matrix[offy][offx] >= matrix[p.1][p.0]
                    && !basin.contains(&(offx, offy))
                {
                    to_visit.push((offx, offy));
                }
            }
            basin.insert(p);
        }
        basins.push(basin);
    }
    basins
}

fn print_basins(matrix: &Vec<Vec<u32>>) {
    let lowpoints = get_lowpoints(matrix);
    let basins = get_basins(matrix);
    for y in 0..matrix.len() {
        'y: for x in 0..matrix[y].len() {
            if lowpoints.contains(&(x, y)) {
                print!("{}{}", color::Bg(color::Reset), matrix[y][x]);
            } else {
                for b in &basins {
                    if b.contains(&(x, y)) {
                        print!("{}{}", color::Bg(color::Green), matrix[y][x]);
                        continue 'y;
                    }
                }
                print!("{}{}", color::Bg(color::Red), matrix[y][x]);
            }
        }
        println!("{}", color::Bg(color::Reset));
    }
    println!("{}", color::Bg(color::Reset));
}

fn solve1(matrix: &Vec<Vec<u32>>) -> u32 {
    let mut risk_sum = 0;
    let lowpoints = get_lowpoints(matrix);
    for p in lowpoints {
        risk_sum += 1 + matrix[p.1][p.0];
    }
    risk_sum
}

fn solve2(matrix: &Vec<Vec<u32>>) -> usize {
    let mut basins = get_basins(matrix);

    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    basins[0].len() * basins[1].len() * basins[2].len()
}

fn main() {
    let input = get_input("input.txt");
    print_basins(&input);
    println!("1: {}", solve1(&input));
    println!("2: {}", solve2(&input));
}
