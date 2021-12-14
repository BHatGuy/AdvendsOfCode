use std::fs;
use text_io::scan;
use std::collections::HashMap;
use std::cmp;

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

fn get_rule<'a>(a: &str, rules: &'a Vec<(String, String)>) -> Option<&'a String> {
    for r in rules {
        if a == r.0 {
            return Some(&r.1);
        }
    }
    None
}


fn solve(inp: &InputType, steps: usize) -> usize {
    let mut template = inp.0.clone();
    let rules = &inp.1;
    for _ in 0..steps {
        let mut next_template = template.clone();
        let mut inserted = 0;
        for i in 0..template.len() - 1 {
            let pair = &template[i..(i + 2)];
            if let Some(a) = get_rule(pair, rules) {
                let c = a.chars().next().unwrap();
                next_template.insert(i + 1 + inserted, c);
                inserted += 1;
            }
        }
        template = next_template;
    }
    let mut count = HashMap::new();
    for c in template.chars(){
        let counter = count.entry(c).or_insert(0);
        *counter += 1;
       
    }
    let mut max = 0;
    let mut min = 999999999; // use max
    for v in count.values() {
        max = cmp::max(max, *v);
        min = cmp::min(min, *v);
    }
    max - min
}



fn main() {
    let input = get_input("input.txt");
    println!("1: {}", solve(&input, 10));
    println!("2: {}", solve(&input, 40));
}
