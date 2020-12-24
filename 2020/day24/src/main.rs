extern crate failure;

use failure::{bail, Error};

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Step {
    E(),
    SE(),
    SW(),
    W(),
    NW(),
    NE(),
}

type Coord = (i64, i64);

fn read_input(filename: &str) -> Result<Vec<Vec<Step>>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut inp = Vec::new();
    for line in reader.lines() {
        let mut steps = Vec::new();
        let l = line?;
        let mut chars = l.chars();
        while let Some(c) = chars.next() {
            let s;
            if c == 's' {
                let c = chars.next().expect("No char after s!");
                s = match c {
                    'e' => Step::SE(),
                    'w' => Step::SW(),
                    _ => bail!("Unknown step {}", c),
                }
            } else if c == 'n' {
                let c = chars.next().expect("No char after s!");
                s = match c {
                    'e' => Step::NE(),
                    'w' => Step::NW(),
                    _ => bail!("Unknown step {}", c),
                }
            } else {
                s = match c {
                    'e' => Step::E(),
                    'w' => Step::W(),
                    _ => bail!("Unknown step {}", c),
                }
            }
            steps.push(s);
        }
        inp.push(steps);
    }

    Ok(inp)
}

fn do_step(mut pos: Coord, step: &Step) -> Coord {
    let even = pos.1 % 2 == 0;
    match step {
        Step::E() => pos.0 += 1,
        Step::SE() => {
            pos.1 += 1;
            if !even {
                pos.0 += 1
            }
        }
        Step::SW() => {
            pos.1 += 1;
            if even {
                pos.0 -= 1
            }
        }
        Step::W() => pos.0 -= 1,
        Step::NW() => {
            pos.1 -= 1;
            if even {
                pos.0 -= 1
            }
        }
        Step::NE() => {
            pos.1 -= 1;
            if !even {
                pos.0 += 1
            }
        }
    }
    pos
}

fn lay_tiles(instructions: &Vec<Vec<Step>>) -> HashSet<Coord> {
    let mut blacks = HashSet::new();
    for tile in instructions {
        let mut pos = (0, 0);
        for step in tile {
            pos = do_step(pos, step);
        }
        if blacks.contains(&pos) {
            blacks.remove(&pos);
        } else {
            blacks.insert(pos);
        }
    }
    blacks
}

fn get_neighbours(c: &Coord) -> Vec<Coord> {
    const DIRS: &[Step] = &[
        Step::E(),
        Step::SE(),
        Step::SW(),
        Step::W(),
        Step::NW(),
        Step::NE(),
    ];
    let mut neigbours = Vec::new();

    for d in DIRS {
        neigbours.push(do_step(*c, d));
    }

    neigbours
}

fn count_actives(tile: &Coord, tiles: &HashSet<Coord>) -> usize {
    get_neighbours(tile)
        .iter()
        .filter(|t| tiles.contains(*t))
        .count()
}

fn calc_next_tiles(old_tiles: &HashSet<Coord>) -> HashSet<Coord> {
    let mut new_tiles = old_tiles.clone();

    // Deactivate:
    for tile in old_tiles {
        let ncount = count_actives(tile, old_tiles);

        if ncount == 0 || ncount > 2 {
            new_tiles.remove(tile);
        }
    }

    // Activate:

    // get inactive neigbours
    let inactives: Vec<Coord> = old_tiles
        .iter()
        .map(|c| get_neighbours(c))
        .fold(Vec::new(), |mut a, mut b| {
            a.append(&mut b);
            a
        })
        .into_iter()
        .filter(|a| !old_tiles.contains(a))
        .collect();
    for tile in inactives {
        let ncount = count_actives(&tile, old_tiles);
        if ncount == 2 {
            new_tiles.insert(tile);
        }
    }
    new_tiles
}

fn solve1(instructions: &Vec<Vec<Step>>) -> usize {
    let tiles = lay_tiles(instructions);
    //println!("{:#?}", blacks);
    tiles.len()
}

fn solve2(instructions: &Vec<Vec<Step>>) -> usize {
    let mut tiles = lay_tiles(instructions);
    for _ in 0..100 {
        tiles = calc_next_tiles(&tiles);
    }
    tiles.len()
}

fn main() -> Result<(), Error> {
    let inp = read_input("input")?;
    println!("1: {:#?}", solve1(&inp));
    let inp = read_input("input")?;
    println!("2: {:#?}", solve2(&inp));
    Ok(())
}
