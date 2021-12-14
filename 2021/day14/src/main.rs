use std::cmp;
use std::collections::HashMap;
use std::fs;
use text_io::scan;

type InputType = (String, Vec<(String, String)>);

fn get_input(name: &str) -> InputType {
    let buf = fs::read_to_string(name).unwrap();
    let mut lines = buf.trim_end().split("\n");
    let mut input = Vec::new();
    let start = lines.next().unwrap();
    lines.next();
    for l in lines {
        let left: String;
        let right: String;
        scan!(l.bytes() => "{} -> {}", left, right);

        input.push((left, right));
    }
    (start.to_string(), input)
}

fn _expand<'a>(a: &str, rules: &'a Vec<(String, String)>) -> Option<&'a String> {
    for r in rules {
        if a == r.0 {
            return Some(&r.1);
        }
    }
    None
}

fn expand_pairs(a: &str, rules: &Vec<(String, String)>) -> Option<(String, String)> {
    for r in rules {
        if a == r.0 {
            assert_eq!(a.len(), 2);
            let first = a[0..1].to_string() + &r.1;
            let last = r.1.to_string() + &a[1..2];
            return Some((first, last));
        }
    }
    None
}

fn _solve(inp: &InputType, steps: usize) -> usize {
    let mut template = inp.0.clone();
    let rules = &inp.1;
    for _ in 0..steps {
        let mut next_template = template.clone();
        let mut inserted = 0;
        for i in 0..template.len() - 1 {
            let pair = &template[i..(i + 2)];
            if let Some(a) = _expand(pair, rules) {
                let c = a.chars().next().unwrap();
                next_template.insert(i + 1 + inserted, c);
                inserted += 1;
            }
        }
        template = next_template;
    }
    let mut count = HashMap::new();
    for c in template.chars() {
        let counter = count.entry(c).or_insert(0);
        *counter += 1;
    }
    let mut max = usize::MIN;
    let mut min = usize::MAX;
    for v in count.values() {
        max = cmp::max(max, *v);
        min = cmp::min(min, *v);
    }
    max - min
}

fn solve_fast(inp: &InputType, steps: usize) -> u128 {
    let template = &inp.0;
    let rules = &inp.1;
    let mut pairs = HashMap::new();
    for i in 0..template.len() - 1 {
        let pair = &template[i..(i + 2)];
        let counter = pairs.entry(pair.to_string()).or_insert(0);
        *counter += 1;
    }
    for _ in 0..steps {
        let mut pairs_next = HashMap::new();
        for (p, c) in pairs {
            let (first, last) = expand_pairs(&p, rules).unwrap();
            let counter = pairs_next.entry(first).or_insert(0);
            *counter += c;

            let counter = pairs_next.entry(last).or_insert(0);
            *counter += c;
        }
        pairs = pairs_next;
    }

    let mut count = HashMap::new();
    count.insert(template.chars().last().unwrap(), 1);
    for (k, v) in pairs {
        let c = k.chars().next().unwrap();
        let counter = count.entry(c).or_insert(0);
        *counter += v;
    }

    let mut max = u128::MIN;
    let mut min = u128::MAX;
    for v in count.values() {
        max = cmp::max(max, *v);
        min = cmp::min(min, *v);
    }
    max - min
}

fn main() {
    let input = get_input("input.txt");
    println!("1: {}", solve_fast(&input, 10));
    println!("2: {}", solve_fast(&input, 40));
}
