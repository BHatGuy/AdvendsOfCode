use std::fs;
use text_io::scan;

type Input = (Vec<(String, String)>, String);

fn read_input(name: &str) -> Input {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut rules = Vec::new();
    for l in lines {
        if l == "" {
            break;
        }
        let mut rule = l.split(" => ");
        let lhs = rule.next().unwrap();
        let rhs = rule.next().unwrap();
        rules.push((lhs.to_owned(), rhs.to_owned()));
    }
    let last = buf.trim_end().split("\n").last().unwrap();

    (rules, last.to_owned())
}

fn main() {
    let input = read_input("input.txt");
    println!("{input:?}")
}
