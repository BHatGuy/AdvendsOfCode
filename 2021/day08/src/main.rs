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
            .map(|x| String::from(x))
            .collect();
        let inputs: Vec<String> = iter
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| String::from(x))
            .collect();
        input.push((uniques, inputs));
    }
    input
}

fn solve1(inp: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    let mut count = 0;
    for segment in inp {
        dbg!(&segment.1);

        count += segment
            .1
            .iter()
            .filter(|x| x.len() == 1 || x.len() == 4 || x.len() == 3 || x.len() == 7)
            .count();
    }

    count
}

fn solve2(inp: &Vec<(Vec<String>, Vec<String>)>) -> u64 {
    0
}

fn main() {
    let input = get_input("testinput.txt");
    println!("1: {}", solve1(&input));
    // println!("2: {}", solve2(&input));
}
