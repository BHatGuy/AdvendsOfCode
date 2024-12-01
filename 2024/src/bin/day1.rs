use std::fs;
use std::iter;
use text_io::scan;

fn read_input(name: &str) -> (Vec<u32>, Vec<u32>) {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for l in lines {
        let a;
        let b;

        scan!(l.bytes() => "{} {}", a, b);
        list1.push(a);
        list2.push(b);
    }

    (list1, list2)
}

fn main() {
    let (mut list1, mut list2) = read_input("input1.txt");
    list1.sort();
    list2.sort();

    let mut sum = 0;
    for (a, b) in iter::zip(&list1, &list2) {
        let diff = if a > b { a - b } else { b - a };
        sum += diff;
    }
    println!("{}", sum);

    let mut similarity = 0;
    for id in list1 {
        let occurences = list2.iter().filter(|x| **x == id).count();
        similarity += occurences * usize::try_from(id).expect("id (u32) does not fit into a usize");
    }
    println!("{similarity}");
}
