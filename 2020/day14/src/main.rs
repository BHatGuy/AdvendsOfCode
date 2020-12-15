use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
enum Inst {
    Mem(u64, u64),
    Mask(String),
}

fn read_input(filename: &str) -> io::Result<Vec<Inst>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut inp = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("mask") {
            let s: Vec<&str> = line.split(" = ").collect();
            inp.push(Inst::Mask(String::from(s[1])))
        } else if line.starts_with("mem") {
            let s: Vec<&str> = line.split(" = ").collect();
            let addr = u64::from_str_radix(&s[0].replace("mem[", "").replace("]", ""), 10)
                .ok()
                .unwrap();
            let val = u64::from_str_radix(s[1], 10).ok().unwrap();
            inp.push(Inst::Mem(addr, val));
        }
    }
    Ok(inp)
}

// From https://stackoverflow.com/questions/40718975/how-to-get-every-subset-of-a-vector-in-rust
fn powerset<T>(s: &[T]) -> Vec<Vec<&T>> {
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element)
                .collect()
        })
        .collect()
}

fn solve1(prog: &Vec<Inst>) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = &"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_owned();
    for ins in prog {
        if let Inst::Mask(m) = ins {
            mask = m;
        } else if let Inst::Mem(addr, mut val) = ins {
            for (i, c) in mask.chars().rev().enumerate() {
                if c == '1' {
                    let m = 1 << i;
                    val = val | m;
                }
                if c == '0' {
                    let m = 1 << i;
                    val = val & (!m);
                }
            }
            mem.insert(addr, val);
        }
    }
    let mut sum = 0;
    for v in mem.values() {
        sum += v;
    }
    sum
}

fn solve2(prog: &Vec<Inst>) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = &"000000000000000000000000000000000000".to_owned();
    for ins in prog {
        if let Inst::Mask(m) = ins {
            mask = m;
        } else if let Inst::Mem(mut addr, val) = ins {
            let mut floatings = Vec::new();
            for (i, c) in mask.chars().rev().enumerate() {
                if c == '1' {
                    let m = 1 << i;
                    addr |= m;
                }
                if c == 'X' {
                    floatings.push(i);
                }
            }
            for sub in powerset(&floatings) {
                let mut newaddr = addr;
                for bit in sub {
                    let m = 1 << bit;
                    newaddr ^= m;
                }
                mem.insert(newaddr, *val);
            }
        }
    }
    let mut sum = 0;
    for v in mem.values() {
        sum += v;
    }
    sum
}

fn main() {
    let inp = read_input("input").unwrap();
    println!("Solution 1: {}", solve1(&inp));
    println!("Solution 2: {}", solve2(&inp));
}
