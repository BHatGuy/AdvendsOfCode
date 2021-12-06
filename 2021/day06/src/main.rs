use std::fs;

fn get_input(name: &str) -> Vec<u8> {
    let buf = fs::read_to_string(name).unwrap();
    let line = buf.split("\n").next().unwrap();
    let mut input = Vec::new();

    for s in line.split(",") {
        input.push(s.parse::<u8>().unwrap());
    }
    return input;
}



fn simulate(inp: &Vec<u8>, days: u16) -> u64 {
    let mut slots = [0u64; 9];
    for fish in inp {
        slots[(*fish) as usize] += 1; 
    }
    for _ in 0..days {
        let mut new_slots = [0u64; 9];
        for i in 1..9 {
            new_slots[i-1] = slots[i]; 
        }
        new_slots[6] += slots[0];
        new_slots[8] += slots[0];
        slots = new_slots;

    }
    slots.iter().sum()
}

fn solve1(inp: &Vec<u8>) -> u64 {
    simulate(inp, 80)
}

fn solve2(inp: &Vec<u8>) -> u64 {
    simulate(inp, 256)
}

fn main() {
    let input = get_input("input.txt");
    println!("1: {}", solve1(&input));
    println!("2: {}", solve2(&input));
}
