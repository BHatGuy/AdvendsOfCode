use std::{collections::HashSet, fs};

fn read_puzzle(name: &str) -> Vec<Vec<char>> {
    let buf = fs::read_to_string(name).expect("Cant read input");
    buf.trim_end()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect()
}

fn main() {
    let map = read_puzzle("input12.txt");

    let mut visited = HashSet::new();
    let mut cost = 0;
    let mut better_cost = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if visited.contains(&(row, col)) {
                continue;
            }
            let crop = map[row][col];
            let mut fringe = vec![(row, col)];
            visited.insert((row, col));
            let mut area = 0;
            let mut perimeter = 0;
            let mut corners = 0;
            while fringe.len() > 0 {
                let (row, col) = fringe.pop().unwrap();
                area += 1;
                for (dr, dc) in vec![(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                    let row = row as i32 + dr;
                    let col = col as i32 + dc;
                    if row < 0
                        || col < 0
                        || row as usize >= map.len()
                        || col as usize >= map[row as usize].len()
                    {
                        perimeter += 1;
                        continue;
                    }
                    let row = row as usize;
                    let col = col as usize;
                    if map[row][col] == crop {
                        if !visited.contains(&(row, col)) {
                            visited.insert((row, col));
                            fringe.push((row, col));
                        }
                    } else {
                        perimeter += 1;
                    }
                }
                for (a, b, c) in vec![
                    ((0, -1), (-1, -1), (-1, 0)),
                    ((-1, 0), (-1, 1), (0, 1)),
                    ((0, 1), (1, 1), (1, 0)),
                    ((1i32, 0i32), (1i32, -1i32), (0i32, -1i32)),
                ] {
                    let a = [row as i32 + a.0, col as i32 + a.1];
                    let b = [row as i32 + b.0, col as i32 + b.1];
                    let c = [row as i32 + c.0, col as i32 + c.1];
                    #[rustfmt::skip]
                    let a_same = *map.get(a[0] as usize).unwrap_or(&vec![]).get(a[1] as usize).unwrap_or(&'.') == crop;
                    #[rustfmt::skip]
                    let b_same = *map.get(b[0] as usize).unwrap_or(&vec![]).get(b[1] as usize).unwrap_or(&'.') == crop;
                    #[rustfmt::skip]
                    let c_same = *map.get(c[0] as usize).unwrap_or(&vec![]).get(c[1] as usize).unwrap_or(&'.') == crop;
                    if !a_same && !c_same {
                        corners += 1;
                    }
                    if a_same && !b_same && c_same {
                        corners += 1;
                    }
                }
            }

            cost += area * perimeter;
            better_cost += area * corners;
        }
    }
    println!("{cost}");
    println!("{better_cost}");
}
