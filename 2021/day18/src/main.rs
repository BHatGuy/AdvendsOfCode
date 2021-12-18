use std::fmt;
use std::fs;
use std::ops::Add;

#[derive(Debug, Clone)]
enum SFN {
    Pair(Box<SFN>, Box<SFN>),
    Number(u64),
}

impl fmt::Display for SFN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SFN::Pair(a, b) => write!(f, "[{}, {}]", a, b),
            SFN::Number(a) => write!(f, "{}", a),
        }
    }
}

impl Add for SFN {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Pair(Box::new(self), Box::new(other))
    }
}


impl SFN {
    fn reduce(&self) -> SFN {
        
        SFN::Number(0)
    }
}

type InputType = Vec<SFN>;

fn parse_sfn(num: &str) -> (SFN, usize) {
    let all = num;
    if !num.starts_with("[") {
        let digit = &num[0..1];
        let digit = SFN::Number(digit.parse().expect(digit));
        return (digit, 1);
    }

    let num = &num[1..];
    let (e1, len) = parse_sfn(num);
    let num = &num[len..];
    if !num.starts_with(",") {
        panic!("no ,");
    }
    let num = &num[1..];
    let (e2, len) = parse_sfn(num);
    let num = &num[len..];
    if !num.starts_with("]") {
        panic!("no ]");
    }
    let num = &num[1..];

    (SFN::Pair(Box::new(e1), Box::new(e2)), all.len() - num.len())
}

fn get_input(name: &str) -> InputType {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut input = Vec::new();

    for l in lines {
        let (num, _) = parse_sfn(l);
        input.push(num);
    }
    input
}

fn solve1(numbers: &InputType) -> i64 {
    let mut sum = numbers[0].clone();
    for num in numbers.iter().skip(1) {
        sum = sum + num.clone();
    }
    println!("{}", sum);
    0
}

fn solve2(_: &InputType) -> u64 {
    0
}

fn main() {
    let input = get_input("input.txt");
    println!("1: {}", solve1(&input));
    println!("2: {}", solve2(&input));
}
