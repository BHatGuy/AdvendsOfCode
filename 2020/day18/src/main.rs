use std::fs::File;
use std::io::{self, BufRead, BufReader};
use Exp::*;

#[derive(Debug)]
enum Exp {
    Add(Box<Exp>, Box<Exp>),
    Mul(Box<Exp>, Box<Exp>),
    Val(u32),
    E(),
}

struct Parser {
    tokens: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(tokens: String) -> Self {
        Self {
            tokens: tokens.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> char {
        self.tokens[self.pos]
    }

    fn eat(&mut self, expected: char) {
        assert_eq!(expected, self.peek());
        self.pos += 1;
    }

    fn next(&mut self) -> char {
        let c = self.tokens[self.pos];
        self.pos += 1;
        c
    }

    fn val(&mut self) -> Exp {
        let c = self.next();
        Val(c.to_digit(10).unwrap())
    }

    

    fn exp(&self) -> Exp {
        E()
    }


}

fn eval(exp: &Exp) -> u32 {
    match exp {
        Val(x) => *x,
        Add(a, b) => eval(a) + eval(b),
        Mul(a, b) => eval(a) * eval(b),
        E() => panic!(),
    }
}

fn parse_input(filename: &str) -> io::Result<Vec<Exp>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut exps = Vec::new();
    for line in reader.lines() {
        let mut line = line?;
        line = line.replace(" ", "");
        exps.push(parse_exp(&mut line.chars().rev()));
    }
    Ok(exps)
}

static TESTS: &[(&str, u32)] = &[
    ("1 + 2 * 3 + 4 * 5 + 6", 71),
    ("2 * 3 + (4 * 5)", 26),
    ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
    ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
    ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
];

fn main() -> io::Result<()> {
    for (st, val) in TESTS {
        let e = parse_exp(&mut st.chars());
        let x = eval(&e);
        println!("{} = {}, soll({}) Baum: {:?}", st, x, val, e);
    }
    println!(
        "{:?}",
        parse_input("input")?
            .iter()
            .map(|e| eval(e))
            .fold(0, |a, b| a + b)
    );
    Ok(())
}
