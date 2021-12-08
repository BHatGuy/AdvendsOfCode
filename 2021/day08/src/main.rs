use itertools::Itertools;
use std::collections::{hash_map, HashSet};
use std::fs;

fn get_input(name: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.split("\n");
    let mut input = Vec::new();

    for l in lines {
        if l.len() == 0 {
            break;
        }
        let mut iter = l.split("|");
        let uniques: Vec<String> = iter
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.chars().sorted().collect())
            .collect();
        let inputs: Vec<String> = iter
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.chars().sorted().collect())
            .collect();
        input.push((uniques, inputs));
    }
    input
}

fn solve1(inp: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    let mut count = 0;
    for display in inp {
        count += display
            .1
            .iter()
            .filter(|x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7)
            .count();
    }

    count
}

fn conatins(a: &str, b: &str) -> bool {
    let set_a: HashSet<char, hash_map::RandomState> = HashSet::from_iter(a.chars());
    let set_b: HashSet<char, hash_map::RandomState> = HashSet::from_iter(b.chars());
    set_a.is_superset(&set_b)
}

fn solve2(inp: &Vec<(Vec<String>, Vec<String>)>) -> u64 {
    let mut acc = 0;
    for display in inp {
        let mut uniques = display.0.clone();
        let mut mapping: [String; 10] = Default::default();
        // find the obvoiuse (via length) 1->len(2), 4->len(4), 7->len(3) 8->len(7)
        mapping[1] = uniques
            .iter()
            .filter(|x| x.len() == 2)
            .last()
            .unwrap()
            .clone();
        mapping[4] = uniques
            .iter()
            .filter(|x| x.len() == 4)
            .last()
            .unwrap()
            .clone();
        mapping[7] = uniques
            .iter()
            .filter(|x| x.len() == 3)
            .last()
            .unwrap()
            .clone();
        mapping[8] = uniques
            .iter()
            .filter(|x| x.len() == 7)
            .last()
            .unwrap()
            .clone();

        uniques.retain(|x| *x != mapping[1]);
        uniques.retain(|x| *x != mapping[4]);
        uniques.retain(|x| *x != mapping[7]);
        uniques.retain(|x| *x != mapping[8]);

        // 2,3,5 have length 5
        // 0,6,9 have length 6

        // of 2,3,5 only 3 contains 1
        mapping[3] = uniques
            .iter()
            .filter(|x| x.len() == 5)
            .filter(|x| conatins(x, &mapping[1]))
            .last()
            .unwrap()
            .clone();
        uniques.retain(|x| *x != mapping[3]);

        // 0,6,9 only 9 contains 4
        mapping[9] = uniques
            .iter()
            .filter(|x| x.len() == 6)
            .filter(|x| conatins(x, &mapping[4]))
            .last()
            .unwrap()
            .clone();
        uniques.retain(|x| *x != mapping[9]);

        // of 2,5 only 5 is contained by 9
        mapping[5] = uniques
            .iter()
            .filter(|x| x.len() == 5)
            .filter(|x| conatins(&mapping[9], x))
            .last()
            .unwrap()
            .clone();
        uniques.retain(|x| *x != mapping[5]);

        mapping[2] = uniques
            .iter()
            .filter(|x| x.len() == 5)
            .filter(|x| !conatins(&mapping[9], x))
            .last()
            .unwrap()
            .clone();
        uniques.retain(|x| *x != mapping[2]);

        // of 0,6 only 0 contains 1
        mapping[0] = uniques
            .iter()
            .filter(|x| x.len() == 6)
            .filter(|x| conatins(x, &mapping[1]))
            .last()
            .unwrap()
            .clone();
        uniques.retain(|x| *x != mapping[0]);

        mapping[6] = uniques
            .iter()
            .filter(|x| x.len() == 6)
            .filter(|x| !conatins(x, &mapping[1]))
            .last()
            .unwrap()
            .clone();
        uniques.retain(|x| *x != mapping[6]);

        let mut num = 0;
        let mut pow = 1;
        for digit in display.1.iter().enumerate().rev() {
            let pos = mapping
                .clone()
                .into_iter()
                .position(|x| x == *digit.1)
                .unwrap();
            num += pow * pos;
            pow *= 10;
        }

        acc += num;
    }

    acc.try_into().unwrap()
}

fn main() {
    let input = get_input("input.txt");
    println!("1: {}", solve1(&input));
    println!("2: {}", solve2(&input));
}
