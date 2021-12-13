use std::cmp;
use std::fs;
use std::iter;
use text_io::scan;
type InputType = Vec<Vec<bool>>;

fn get_input(name: &str) -> InputType {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut dots = Vec::new();
    let mut maxx = 0;
    let mut maxy = 0;
    for l in lines {
        if l.len() < 1 {
            continue;
        }
        if l.starts_with("fold") {
            break;
        }
        let x: usize;
        let y: usize;
        scan!(l.bytes() => "{},{}", x, y);
        dots.push((x, y));
        maxx = cmp::max(maxx, x);
        maxy = cmp::max(maxy, y);
    }
    let mut input: InputType = iter::repeat(iter::repeat(false).take(maxx+1).collect())
        .take(maxy+1)
        .collect();

    for d in dots {
        input[d.1][d.0] = true;
    }
    input
}

fn fold_y(matrix: &mut InputType, y: usize) {
    let mut i = 0;
    while matrix.len() != y + 1 {
        let row = matrix.remove(y+1);

        let target = y - 1 - i;

        for (k, b) in row.iter().enumerate() {
            matrix[target][k] |= b;
        }

        i += 1;
    }
    matrix.remove(y);
}

fn print_sheet(matrix: &InputType){
    for row in matrix {
        for &d in row {
            if d {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn solve1(mut matrix: InputType) -> usize {
    print_sheet(&matrix);
    fold_y(&mut matrix, 7);
    print_sheet(&matrix);
    0
}

fn solve2(matrix: InputType) -> u64 {
    0
}

fn main() {
    let input = get_input("test.txt");
    println!("1: {}", solve1(input.clone()));
    println!("2: {}", solve2(input));
}
