use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::fs;
use text_io::scan;

fn read_input(name: &str) -> Vec<Vec<i64>> {
    let buf = fs::read_to_string(name).unwrap();
    let mut name_map = HashMap::new();
    let mut name_index = 0usize;

    let lines = buf.trim_end().split("\n");
    for l in lines {
        let alice: String;
        let bob: String;
        let _direction: String;
        let _value: i64;

        scan!(l.bytes() => "{} would {} {} happiness units by sitting next to {}.", alice, _direction, _value, bob);

        if !name_map.contains_key(&alice) {
            name_map.insert(alice.clone(), name_index);
            name_index += 1;
        }
        if !name_map.contains_key(&bob) {
            name_map.insert(bob.clone(), name_index);
            name_index += 1;
        }
    }

    let mut dist_matrix = vec![vec![0i64; name_map.len()]; name_map.len()];

    let lines = buf.trim_end().split("\n");
    for l in lines {
        let alice: String;
        let bob: String;
        let direction: String;
        let mut value: i64;

        scan!(l.bytes() => "{} would {} {} happiness units by sitting next to {}.", alice, direction, value, bob);

        if direction == "lose" {
            value = -value;
        } else if direction != "gain" {
            panic!("Unknown direction");
        }

        dist_matrix[name_map[&alice]][name_map[&bob]] = value;
    }
    return dist_matrix;
}

fn solve(input: &Vec<Vec<i64>>) {
    let persons: Vec<usize> = (0..input.len()).collect();
    let mut maxcost = i64::MIN;

    for perm in persons.iter().permutations(persons.len()) {
        let mut cost = 0;

        for i in 0..perm.len() - 1 {
            cost += input[*perm[i]][*perm[i + 1]];
            cost += input[*perm[i + 1]][*perm[i]];
        }
        cost += input[*perm[0]][*perm[perm.len() - 1]];
        cost += input[*perm[perm.len() - 1]][*perm[0]];

        maxcost = cmp::max(maxcost, cost);
    }
    println!("{maxcost}");
}

fn add_me(input: &mut Vec<Vec<i64>>) {
    for v in input.iter_mut() {
        v.push(0);
    }
    let me = vec![0; input.len() + 1];
    input.push(me);
}

fn main() {
    let mut input = read_input("input.txt");
    solve(&input);
    add_me(&mut input);
    solve(&input);
}
