use std::collections::HashMap;
static INPUT: &[u32] = &[14, 1, 17, 0, 3, 20];

fn solve(n: usize) -> u32 {
    let mut numbers = Vec::from(INPUT);
    let mut last_positions = HashMap::new();
    let mut turn = 0;
    for i in 0..(numbers.len() - 1) {
        last_positions.insert(numbers[i], turn);
        turn += 1;
    }
    turn += 1;
    for _ in 0..(n - numbers.len()) {
        let last = *numbers.last().unwrap();

        let pos = last_positions.get(&last);
        let next = match pos {
            Some(x) => numbers.len() - x - 1,
            None => 0,
        };
        numbers.push(next as u32);

        last_positions.insert(last, turn - 1);

        turn += 1;
    }
    return *numbers.last().unwrap();
}

fn main() {
    println!("Solution 1: {}", solve(2020));
    println!("Solution 2: {}", solve(30000000));
}
