use std::fs;

fn read_calculations(name: &str) -> Vec<(u64, Vec<u64>)> {
    let buf = fs::read_to_string(name).expect("Cant read input");
    let lines = buf.trim_end().split("\n");
    let mut calculations = Vec::new();
    for line in lines {
        let all: Vec<u64> = line
            .replace(":", "")
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        calculations.push((all[0], all[1..].to_owned()));
    }
    calculations
}

fn main() {
    let calculations = read_calculations("input7.txt");

    let mut sum = 0;

    for (result, numbers) in &calculations {
        let mut possibilities = vec![numbers[0]];
        for num in &numbers[1..] {
            let mut new_possibilities = Vec::new();
            for p in possibilities {
                let add = p + num;
                if add <= *result {
                    new_possibilities.push(add);
                }
                let mul = p * num;
                if mul <= *result {
                    new_possibilities.push(mul);
                }
            }
            possibilities = new_possibilities;
        }
        if possibilities.contains(&result) {
            sum += result;
        }
    }

    println!("{sum}");

    let mut sum = 0;

    for (result, numbers) in &calculations {
        let mut possibilities = vec![numbers[0]];
        for num in &numbers[1..] {
            let mut new_possibilities = Vec::new();
            for p in possibilities {
                let add = p + num;
                if add <= *result {
                    new_possibilities.push(add);
                }
                let mul = p * num;
                if mul <= *result {
                    new_possibilities.push(mul);
                }
                let concat: u64 = (p.to_string() + &num.to_string()).parse().unwrap();
                if concat <= *result {
                    new_possibilities.push(concat);
                }
            }
            possibilities = new_possibilities;
        }
        if possibilities.contains(&result) {
            sum += result;
        }
    }

    println!("{sum}");
}
