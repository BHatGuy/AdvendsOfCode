use std::fs;
use text_io::scan;

type Sue = [Option<usize>; 10];

fn add_compound(compound: &str, count: usize, sue: &mut Sue) {
    let index = match compound {
        "children" => 0,
        "cats" => 1,
        "samoyeds" => 2,
        "pomeranians" => 3,
        "akitas" => 4,
        "vizslas" => 5,
        "goldfish" => 6,
        "trees" => 7,
        "cars" => 8,
        "perfumes" => 9,
        _ => panic!("Unknown Compound {compound}"),
    };
    sue[index] = Some(count);
}

fn read_input(name: &str) -> Vec<Sue> {
    let buf = fs::read_to_string(name).unwrap();
    let mut sues = Vec::new();
    let lines = buf.trim_end().split("\n");
    for l in lines {
        let number: usize;
        let comp1: String;
        let comp2: String;
        let comp3: String;
        let count1: usize;
        let count2: usize;
        let count3: usize;

        scan!(l.bytes() => "Sue {}: {}: {}, {}: {}, {}: {}", number, comp1, count1, comp2, count2, comp3, count3);

        let mut sue = [None; 10];
        add_compound(&comp1, count1, &mut sue);
        add_compound(&comp2, count2, &mut sue);
        add_compound(&comp3, count3, &mut sue);

        sues.push(sue);
    }

    sues
}

fn solve(needle: &Sue, haystack: &Vec<Sue>) {
    for (i, sue) in haystack.iter().enumerate() {
        let res = sue
            .iter()
            .zip(needle)
            .all(|(&a, b)| a.is_none() || a.unwrap() == b.unwrap());
        if res {
            println!("{i}");
            return;
        }
    }
}

fn solve2(needle: &Sue, haystack: &Vec<Sue>) {
    'outer: for (i, sue) in haystack.iter().enumerate() {
        for index in [0, 2, 4, 5, 8, 9] {
            if sue[index].is_some() && sue[index] != needle[index] {
                continue 'outer;
            }
        }
        if sue[1].is_some() && sue[1] <= needle[1] {
            continue 'outer;
        }
        if sue[7].is_some() && sue[7] <= needle[7] {
            continue 'outer;
        }
        if sue[3].is_some() && sue[3] >= needle[3] {
            continue 'outer;
        }
        if sue[6].is_some() && sue[6] >= needle[6] {
            continue 'outer;
        }
        println!("{i}");
    }
}

fn main() {
    let sues = read_input("input.txt");
    //children: 3
    //cats: 7
    //samoyeds: 2
    //pomeranians: 3
    //akitas: 0
    //vizslas: 0
    //goldfish: 5
    //trees: 3
    //cars: 2
    //perfumes: 1
    let sue = [
        Some(3),
        Some(7),
        Some(2),
        Some(3),
        Some(0),
        Some(0),
        Some(5),
        Some(3),
        Some(2),
        Some(1),
    ];

    solve(&sue, &sues);
    solve2(&sue, &sues);
}
