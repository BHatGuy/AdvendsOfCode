use std::fmt;
use std::fs;
use std::ops::Add;
use SFN::{Number, Pair};

#[derive(Debug, Clone, PartialEq, Eq)]
enum SFN {
    Pair(Box<SFN>, Box<SFN>),
    Number(u64),
}

impl fmt::Display for SFN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pair(a, b) => write!(f, "[{},{}]", a, b),
            Number(a) => write!(f, "{}", a),
        }
    }
}

impl Add for SFN {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Pair(Box::new(self), Box::new(other))
    }
}

impl<'a, 'b> Add<&'b SFN> for &'a SFN {
    type Output = SFN;

    fn add(self, other: &'b SFN) -> SFN {
        Pair(Box::new(self.clone()), Box::new(other.clone()))
    }
}

impl SFN {
    fn find_explode(&self, depth: usize) -> Option<SFN> {
        match self {
            Number(_) => {}
            Pair(a, b) => {
                if depth == 4 {
                    return Some(self.clone());
                }
                if let Some(res) = a.find_explode(depth + 1) {
                    return Some(res);
                }
                if let Some(res) = b.find_explode(depth + 1) {
                    return Some(res);
                }
            }
        };
        None
    }

    fn replace(&mut self, target: &SFN, val: &SFN) -> bool{
        match self {
            Number(_) => {}
            Pair(a, b) => {
                if **a == *target {
                    *self = Pair(Box::new(val.clone()), Box::new(*b.clone()));
                    return true;
                } 
                if **b == *target {
                    *self = Pair(Box::new(*a.clone()), Box::new(val.clone()));
                    return true;
                }
                if a.replace(target, val){
                    return true;
                }
                if b.replace(target, val){
                    return true;
                }               

            }
        };
        false
    }



    fn reduce(&mut self) {
        // explode?
        if let Some(p) = self.find_explode(0) {
            println!("{}", p);
            self.replace(&p, &Number(0));
            println!("{}", self);
            
        }
    }
}

type InputType = Vec<SFN>;

fn parse_sfn(num: &str) -> (SFN, usize) {
    let all = num;
    if !num.starts_with("[") {
        let digit = &num[0..1];
        let digit = Number(digit.parse().expect(digit));
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
        sum = &sum + num;
        sum.reduce();
    }
    let (mut test, _) = parse_sfn("[[[[[9,8],1],2],3],4]");
    println!("{}", test);
    test.reduce();
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
