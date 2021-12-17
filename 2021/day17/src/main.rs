use std::cmp::max;
use std::fs;
use std::ops::Range;
use text_io::scan;

type InputType = (Range<i64>, Range<i64>);

fn get_input(name: &str) -> Vec<InputType> {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut input = Vec::new();

    for l in lines {
        let startx: i64;
        let endx: i64;
        let starty: i64;
        let endy: i64;
        scan!(l.bytes() => "target area: x={}..{}, y={}..{}", startx, endx, starty, endy);
        input.push((startx..endx+1, starty..endy+1));
    }
    input
}

fn yeet(mut velx: i64, mut vely: i64, target: &InputType) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut maxy = y;
    loop {
        if target.0.contains(&x) && target.1.contains(&y) {
            return Some(maxy);
        }

        if x < target.0.start && x < target.0.end && velx < 0 {
            return None;
        }

        if x > target.0.start && x > target.0.end && velx > 0 {
            return None;
        }

        if y < target.1.start && y < target.1.end && vely < 0 {
            return None;
        }

        x += velx;
        y += vely;
        maxy = max(y, maxy);
        velx = velx.signum() * (velx.abs() - 1);
        vely -= 1;
    }
}

fn solve1(target: &InputType) -> i64 {
    let mut max_height = i64::MIN;
    // TODO dynamic Ranges
    for x in -100..100 {
        for y in -100..100 {
            if let Some(height) = yeet(x, y, target) {
                max_height = max(height, max_height);
            }
        }
    }
    max_height
}

fn solve2(target: &InputType) -> u64 {
    let mut count = 0;
    // TODO dynamic Ranges
    for x in -1000..1000 {
        for y in -1000..1000 {
            if let Some(_) = yeet(x, y, target) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let input = get_input("input.txt");
    let idx = 1;
    println!("1: {}", solve1(&input[idx]));
    println!("2: {}", solve2(&input[idx]));
}
