use std::fs;

use text_io::scan;

fn read_input(name: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut iter = lines.into_iter();
    let mut rules = Vec::new();
    for l in iter.by_ref() {
        if l == "" {
            break;
        }
        let a;
        let b;
        scan!(l.bytes() => "{}|{}", a, b);
        rules.push((a, b));
    }

    let mut manuals = Vec::new();
    for l in iter {
        let pages = l.split(",").map(|x| str::parse(x).unwrap()).collect();

        manuals.push(pages);
    }

    (rules, manuals)
}
fn main() {
    let (rules, mut manuals) = read_input("input5.txt");

    let mut ok_sum = 0;
    let mut invalids = Vec::new();
    'outer: for man in manuals.iter_mut() {
        for (a, b) in &rules {
            let index_a = match man.iter().position(|x| x == a) {
                Some(x) => x,
                None => continue,
            };
            let index_b = match man.iter().position(|x| x == b) {
                Some(x) => x,
                None => continue,
            };
            if index_a > index_b {
                invalids.push(man);
                continue 'outer;
            }
        }
        ok_sum += man[man.len() / 2];
    }
    println!("{ok_sum}");

    let mut ok_sum = 0;
    for man in invalids {
        loop {
            let mut swap = false;
            for (a, b) in &rules {
                let index_a = match man.iter().position(|x| x == a) {
                    Some(x) => x,
                    None => continue,
                };
                let index_b = match man.iter().position(|x| x == b) {
                    Some(x) => x,
                    None => continue,
                };
                if index_a > index_b {
                    swap = true;
                    man.swap(index_a, index_b);
                }
            }
            if !swap {
                break;
            }
        }
        ok_sum += man[man.len() / 2];
    }
    println!("{ok_sum}");
}
