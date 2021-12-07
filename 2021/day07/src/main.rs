use std::fs;

fn get_input(name: &str) -> Vec<i64> {
    let buf = fs::read_to_string(name).unwrap();
    let line = buf.split("\n").next().unwrap();
    let mut input = Vec::new();

    for s in line.split(",") {
        input.push(s.parse::<i64>().unwrap());
    }
    input
}

fn calc_fuel(inp: &Vec<i64>, pos: i64) -> i64{
    inp.iter().map(|x| (x-pos).abs()).sum()
}

fn real_calc_fuel(inp: &Vec<i64>, pos: i64) -> i64{
    inp.iter().map(|x| fuel_consumption(*x, pos)).sum()
}

fn fuel_consumption(a: i64, b: i64) -> i64 {
    let d = (a - b).abs();
    d*(d+1) / 2
}

fn solve1(inp: &Vec<i64>) -> i64 {
    let max = *inp.iter().max().unwrap();
    let min = *inp.iter().min().unwrap();
    let mut min_fuel = std::i64::MAX;
    for pos in min..(max+1){
        let f = calc_fuel(inp, pos);
        if f < min_fuel{
            min_fuel = f;
            // min_pos = pos
        }
    }
    min_fuel
}

fn solve2(inp: &Vec<i64>) -> i64 {
    let max = *inp.iter().max().unwrap();
    let min = *inp.iter().min().unwrap();
    let mut min_fuel = std::i64::MAX;
    for pos in min..(max+1){
        let f = real_calc_fuel(inp, pos);
        if f < min_fuel{
            min_fuel = f;
        }
    }
    min_fuel
}

fn main() {
    let input = get_input("input.txt");
    println!("1: {}", solve1(&input));
    println!("2: {}", solve2(&input));
}



