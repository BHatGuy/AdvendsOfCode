use regex::Regex;
use std::fs;

fn read_memory(name: &str) -> String {
    let buf = fs::read_to_string(name).expect("Cant read input");
    buf.trim_end().replace("\n", "")
}
fn main() {
    let memory = read_memory("input3.txt");

    let re = Regex::new(r"mul\((\d?\d?\d),(\d?\d?\d)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut total_sum = 0;
    let mut enabled = true;

    for mul in re.captures_iter(memory.as_str()) {
        if mul.get(0).unwrap().as_str() == "do()" {
            enabled = true;
        } else if mul.get(0).unwrap().as_str() == "don't()" {
            enabled = false;
        } else {
            let x: u32 = str::parse(mul.get(1).unwrap().as_str()).unwrap();
            let y: u32 = str::parse(mul.get(2).unwrap().as_str()).unwrap();

            if enabled {
                sum += x * y;
            }
            total_sum += x * y;
        }
    }

    println!("{total_sum}");
    println!("{sum}");
}
