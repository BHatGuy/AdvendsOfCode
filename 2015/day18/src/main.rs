use std::fs;

fn read_input(name: &str) -> Vec<Vec<bool>> {
    let buf = fs::read_to_string(name).unwrap();
    let mut lights = Vec::new();
    let lines = buf.trim_end().split("\n");
    for l in lines {
        let row = l
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("{c}"),
            })
            .collect();
        lights.push(row);
    }

    lights
}

fn count(lights: &Vec<Vec<bool>>, x: i32, y: i32) -> usize {
    let mut count = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if (dx == 0 && dy == 0)
                || x + dx < 0
                || y + dy < 0
                || x + dx >= lights.len().try_into().unwrap()
                || y + dy >= lights.len().try_into().unwrap()
            {
                continue;
            }
            if lights[(y+dy) as usize][(x+dx) as usize] {
                count += 1;
            }

        }
    }
    count
}

fn solve(lights: &Vec<Vec<bool>>, stuck: bool) {
    let mut lights = lights.clone();
    for _ in 0..100 {
        let mut new_lights = Vec::new();
        for y in 0..lights.len() {
            let mut new_row = Vec::new();
            for x in 0..lights[y].len() {
                let l = lights[y][x];
                let count = count(&lights, x as i32, y as i32);

                let n = if l {
                    if count == 2 || count == 3 {
                        true
                    } else {
                        false
                    }
                } else {
                    if count == 3 {
                        true
                    } else {
                        false
                    }
                };

                new_row.push(n);
            }
            new_lights.push(new_row);
        }
        if stuck {
            let l = new_lights.len()- 1;
            new_lights[0][0] = true;
            new_lights[l][0] = true;
            new_lights[0][l] = true;
            new_lights[l][l] = true;
        }
        lights = new_lights;
    }
    let on_count = lights.iter().flatten().filter(|&&x| x).count();
    println!("{on_count}");
}

fn main() {
    let lights = read_input("input.txt");
    solve(&lights, false);
    solve(&lights, true);
}
