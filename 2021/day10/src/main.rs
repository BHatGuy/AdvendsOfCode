use std::fs;

fn get_input(name: &str) -> Vec<String> {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut input = Vec::new();

    for l in lines {
        let row = l.trim().to_string();
        input.push(row);
    }
    input
}

fn score(c: char) -> u128 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("un scorable!"),
    }
}

fn open_to_close(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("no bracket!"),
    }
}

fn is_corrupted(line: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else if c == ')' || c == ']' || c == '}' || c == '>' {
            let tos = stack.pop().unwrap();
            if open_to_close(tos) != c {
                return Some(c);
            }
        } else {
            panic!("illigal character");
        }
    }
    None
}

fn solve1(input: &Vec<String>) -> u128 {
    let mut sum = 0;
    for expression in input {
        if let Some(bad) = is_corrupted(expression) {
            sum += score(bad);
        }
    }
    sum
}

fn solve2(input: &Vec<String>) -> u128 {
    let mut sums = Vec::new();
    let filtered = input.iter().filter(|x| is_corrupted(x).is_none());
    for line in filtered {
        let mut sum = 0;
        let mut stack = Vec::new();
        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
            } else if c == ')' || c == ']' || c == '}' || c == '>' {
                let tos = stack.pop().unwrap();
                if open_to_close(tos) != c {
                    panic!("corrupted line");
                }
            } else {
                panic!("illigal character");
            }
        }
        while stack.len() > 0 {
            let c = stack.pop().unwrap();
            sum *= 5;
            sum += score(c);
        }
        sums.push(sum);
    }
    sums.sort();
    sums[sums.len()/2]
}

fn main() {
    let input = get_input("input.txt");
    println!("1: {}", solve1(&input));
    println!("2: {}", solve2(&input));
}
