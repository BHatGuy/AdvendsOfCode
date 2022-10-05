use itertools::Itertools;
use std::fs;
use text_io::scan;

fn read_input(name: &str) -> Vec<usize> {
    let buf = fs::read_to_string(name).unwrap();
    let mut containers = Vec::new();
    let lines = buf.trim_end().split("\n");
    for l in lines {
        let size: usize;

        scan!(l.bytes() => "{}", size);

        containers.push(size);
    }

    containers
}

fn solve(containers: &Vec<usize>) {
    let mut count = 0;
    let mut min = true;
    for n in 1..containers.len() {
        let c = containers
            .iter()
            .combinations(n)
            .map(|x| x.iter().copied().sum())
            .filter(|&x: &usize| x == 150)
            .count();
        count += c;
        if min && c > 0 {
            min = false;
            println!("{c}");
        }
    }

    println!("{count}");
}

fn main() {
    let containers = read_input("input.txt");
    solve(&containers);
}
