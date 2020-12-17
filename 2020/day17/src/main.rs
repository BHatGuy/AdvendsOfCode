extern crate itertools;
use itertools::Itertools;
use std::collections::HashSet;

type Coord = Vec<i32>;

static INPUT: &[&[char]] = &[
    &['#', '#', '.', '#', '.', '.', '.', '#'],
    &['#', '.', '.', '#', '#', '.', '.', '.'],
    &['.', '.', '.', '.', '#', '.', '.', '#'],
    &['.', '.', '.', '.', '#', '#', '#', '#'],
    &['#', '.', '#', '.', '.', '.', '.', '#'],
    &['#', '#', '#', '.', '#', '.', '#', '.'],
    &['.', '#', '.', '#', '.', '#', '.', '.'],
    &['.', '#', '.', '.', '.', '.', '.', '#'],
];

fn get_start_coords(inp: &[&[char]], dim: usize) -> HashSet<Coord> {
    let mut res = HashSet::new();
    let mut y = 0;
    for row in inp {
        let mut x = 0;
        for cube in *row {
            if *cube == '#' {
                let mut pos = vec![x, y];
                pos.resize(dim, 0);
                res.insert(pos);
            }
            x += 1;
        }
        y += 1;
    }
    res
}

fn get_neighbours(c: &Coord) -> Vec<Coord> {
    let dim = c.len();
    let mut n = Vec::new();
    const DS: [i32; 3] = [-1, 0, 1];

    for dir in (0..dim).map(|_| DS.iter()).multi_cartesian_product() {
        if dir.iter().all(|&&x| x == 0) {
            continue;
        }
        let co: Coord = dir.iter().zip(c).map(|a| *a.0 + a.1).collect();
        n.push(co);
    }

    n
}

fn count_actives(cube: &Coord, cubes: &HashSet<Coord>) -> usize {
    get_neighbours(cube)
        .iter()
        .filter(|c| cubes.contains(*c))
        .count()
}

fn step(old_cubes: &HashSet<Coord>) -> HashSet<Coord> {
    let mut new_cubes = old_cubes.clone();

    // Deactivate:
    for cube in old_cubes {
        let ncount = count_actives(cube, old_cubes);

        if ncount != 2 && ncount != 3 {
            new_cubes.remove(cube);
        }
    }

    // Activate:

    // get inactive neigbours
    let inactives: Vec<Coord> = old_cubes
        .iter()
        .map(|c| get_neighbours(c))
        .fold(Vec::new(), |mut a, mut b| {
            a.append(&mut b);
            a
        })
        .into_iter()
        .filter(|a| !old_cubes.contains(a))
        .collect();
    for cube in inactives {
        let ncount = count_actives(&cube, old_cubes);
        if ncount == 3 {
            new_cubes.insert(cube);
        }
    }
    new_cubes
}

fn solve(dim: usize) -> usize {
    let mut acubes = get_start_coords(INPUT, dim);
    for _i in 0..6 {
        acubes = step(&acubes);
    }
    return acubes.len();
}

fn main() {
    println!("{}", solve(3));
    println!("{}", solve(4));
}
