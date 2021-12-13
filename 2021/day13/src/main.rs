use std::cmp;
use std::fs;
use std::iter;
use text_io::scan;

#[derive(Clone, Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

type InputType = (Vec<Vec<bool>>, Vec<Fold>);

fn get_input(name: &str) -> InputType {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut dots = Vec::new();
    let mut maxx = 0;
    let mut maxy = 0;
    let mut folds = Vec::new();
    for l in lines {
        if l.len() < 1 {
            continue;
        }
        if l.starts_with("fold") {
            if l.starts_with("fold along y") {
                let y;
                scan!(l.bytes() => "fold along y={}", y);
                folds.push(Fold::Y(y));
            } else if l.starts_with("fold along x") {
                let x;
                scan!(l.bytes() => "fold along x={}", x);
                folds.push(Fold::X(x));
            }
            continue;
        }
        let x: usize;
        let y: usize;
        scan!(l.bytes() => "{},{}", x, y);
        dots.push((x, y));
        maxx = cmp::max(maxx, x);
        maxy = cmp::max(maxy, y);
    }
    let mut matrix: Vec<Vec<bool>> = iter::repeat(iter::repeat(false).take(maxx + 1).collect())
        .take(maxy + 1)
        .collect();

    for d in dots {
        matrix[d.1][d.0] = true;
    }
    (matrix, folds)
}

fn fold_y(matrix: &mut Vec<Vec<bool>>, y: usize) {
    let mut i = 0;
    while matrix.len() != y + 1 {
        let row = matrix.remove(y + 1);

        let target = y - 1 - i;

        for (k, b) in row.iter().enumerate() {
            matrix[target][k] |= b;
        }

        i += 1;
    }
    matrix.remove(y);
}

fn fold_x(matrix: &mut Vec<Vec<bool>>, mut x: usize) {
    let rest = matrix[0].len() - x;
    for _ in 1..(rest - x){
        for row in matrix.iter_mut() {
            row.insert(0, false);
        }
        x = x + 1;
    }
    
    for row in matrix {
        let mut i = 0;
        while row.len() != x + 1 {
            let b = row.remove(x + 1);
            let target = x - 1 - i;
            row[target] |= b;
            i += 1;
        }
        row.remove(x);
    }
}

fn print_sheet(matrix: &Vec<Vec<bool>>) {
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

fn solve1(inp: InputType) -> usize {
    let mut matrix = inp.0;
    match inp.1[0] {
        Fold::X(x) => fold_x(&mut matrix, x),
        Fold::Y(y) => fold_y(&mut matrix, y),
    }
    matrix.into_iter().flatten().filter(|x| *x).count()
}

fn solve2(inp: InputType) {
    let mut matrix = inp.0;
    for f in inp.1 {
        match f {
            Fold::X(x) => fold_x(&mut matrix, x),
            Fold::Y(y) => fold_y(&mut matrix, y),
        }
    }
    print_sheet(&matrix);
}

fn main() {
    // BLKJRBAG
    let input = get_input("input.txt");
    println!("1: {}", solve1(input.clone()));
    solve2(input);
}
