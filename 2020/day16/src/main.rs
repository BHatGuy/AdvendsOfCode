
extern crate failure;
use failure::Error;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn read_input(
    filename: &str,
) -> Result<
    (
        Vec<(String, Range<u32>, Range<u32>)>,
        Vec<u32>,
        Vec<Vec<u32>>,
    ),
    Error,
> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut rules = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        if line.len() == 0 {
            break;
        }
        let s: Vec<&str> = line.split(": ").collect();
        let name = s[0].to_owned();
        let r: Vec<&str> = s[1].split(" or ").collect();
        let r1: Vec<&str> = r[0].split("-").collect();
        let r2: Vec<&str> = r[1].split("-").collect();
        let r1min: u32 = r1[0].parse()?;
        let r1max: u32 = r1[1].parse()?;
        let r2min: u32 = r2[0].parse()?;
        let r2max: u32 = r2[1].parse()?;
        rules.push((name, r1min..r1max + 1, r2min..r2max + 1));
    }
    lines.next();
    let line = lines.next().unwrap()?;
    let my_ticket: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
    lines.next();
    lines.next();
    let mut near_tickes = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        let ticket: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        near_tickes.push(ticket)
    }
    Ok((rules, my_ticket, near_tickes))
}

fn is_valid(ticket: &Vec<u32>, rules: &Vec<(String, Range<u32>, Range<u32>)>) -> bool {
    'outer: for val in ticket {
        for (_, r1, r2) in rules {
            if r1.contains(val) || r2.contains(val) {
                continue 'outer;
            }
            return false;
        }
    }
    true
}

fn solve1() -> Result<u32, Error> {
    let (rules, _my, tickets) = read_input("input")?;
    let mut invalval = Vec::new();
    'outer: for val in tickets.iter().flatten() {
        for (_, r1, r2) in &rules {
            if r1.contains(val) || r2.contains(val) {
                continue 'outer;
            }
        }
        invalval.push(*val);
    }
    Ok(invalval.iter().sum())
}

fn solve2() -> Result<u32, Error> {
    let (rules, my, tickets) = read_input("input")?;
    let valids: Vec<&Vec<u32>> = tickets.iter().filter(|t| is_valid(t, &rules)).collect();
    let n = my.len();
    let mut possibilities: HashMap<&String, HashSet<usize>> = HashMap::new();
    for (name, _, _) in rules.iter() {
        possibilities.insert(name, (0..n).collect());
    }
    for ticket in valids {
        for (i, val) in ticket.iter().enumerate() {
            for (name, r1, r2) in rules.iter() {
                if !(r1.contains(val) || r2.contains(val)) {
                    possibilities.get_mut(name).unwrap().remove(&i);
                }
            }
        }
    }
    println!("{:?}", possibilities);
    Ok(0)
}

fn main() -> Result<(), Error> {
    println!("{:?}", solve1()?);
    println!("{:?}", solve2()?);
    Ok(())
}
