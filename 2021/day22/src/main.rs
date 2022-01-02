use std::fs;
use std::ops::Range;
use text_io::scan;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Cuboid {
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
}
type InputType = Vec<(bool, Cuboid)>;

fn get_input(name: &str) -> InputType {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut input = Vec::new();

    for l in lines {
        let start_x;
        let end_x: i32;
        let start_y;
        let end_y: i32;
        let start_z;
        let end_z: i32;
        let pred: String;
        scan!(l.bytes() => "{} x={}..{},y={}..{},z={}..{}", pred, start_x, end_x, start_y, end_y, start_z, end_z);
        let on = match pred.as_str() {
            "on" => true,
            "off" => false,
            _ => panic!("neither on nor off"),
        };

        input.push((
            on,
            Cuboid {
                x: start_x..end_x + 1,
                y: start_y..end_y + 1,
                z: start_z..end_z + 1,
            },
        ));
    }
    input
}

fn solve1(inp: &InputType) -> usize {
    let offset = 50;
    let mut reactor = [[[false; 101]; 101]; 101];
    for step in inp {
        let (
            on,
            Cuboid {
                x: range_x,
                y: range_y,
                z: range_z,
            },
        ) = step;
        if range_x.start > 50
            || range_x.end > 50
            || range_x.start < -50
            || range_x.end < -50
            || range_y.start > 50
            || range_y.end > 50
            || range_y.start < -50
            || range_y.end < -50
            || range_z.start > 50
            || range_z.end > 50
            || range_z.start < -50
            || range_z.end < -50
        {
            continue;
        }
        for x in range_x.clone() {
            for y in range_y.clone() {
                for z in range_z.clone() {
                    reactor[(z + offset) as usize][(y + offset) as usize][(x + offset) as usize] =
                        *on;
                }
            }
        }
    }
    reactor.iter().flatten().flatten().filter(|x| **x).count()
}

fn range_intersect<I: std::cmp::PartialOrd>(a: &Range<I>, b: &Range<I>) -> bool {
    a.contains(&b.start) || b.contains(&a.start)
}

fn intersecting(a: &Cuboid, b: &Cuboid) -> bool {
    range_intersect(&a.x, &b.x) && range_intersect(&a.y, &b.y) && range_intersect(&a.z, &b.z)
}

/** a - b: creates up to 26 new cuboids*/
fn take_outoids(a: &Cuboid, b: &Cuboid) -> Vec<Cuboid> {
    if !intersecting(a, b) {
        return Vec::new();
    }
    let mut partoids = Vec::new();
    let mut xs = vec![a.x.start, a.x.end, b.x.start, b.x.end];
    let mut ys = vec![a.y.start, a.y.end, b.y.start, b.y.end];
    let mut zs = vec![a.z.start, a.z.end, b.z.start, b.z.end];

    xs.sort();
    ys.sort();
    zs.sort();

    let xranges = xs
        .iter()
        .take(xs.len() - 1)
        .zip(xs.iter().skip(1))
        .map(|(a, b)| *a..*b)
        ;
    let yranges = ys
        .iter()
        .take(ys.len() - 1)
        .zip(ys.iter().skip(1))
        .map(|(a, b)| *a..*b)
        ;
    let zranges = zs
        .iter()
        .take(zs.len() - 1)
        .zip(zs.iter().skip(1))
        .map(|(a, b)| *a..*b);

    for x in xranges.clone() {
        for y in yranges.clone() {
            for z in zranges.clone()  {
                if vec![x.is_empty(), y.is_empty(), z.is_empty()].iter().any(|&x| x) {
                    continue;
                }
                let newboid = Cuboid{x: x.clone(), y: y.clone(), z: z.clone()};
                if !intersecting(&newboid, b){
                    partoids.push(newboid);
                }
            }
        }
    }

    partoids
}

fn solve2(inp: &InputType) -> u128 {
    let mut cuboids = HashSet::new();
    for (i, step) in inp.iter().enumerate() {
        let (on, newboid) = step;
        let mut interoids = Vec::new();
        for iboid in &cuboids {
            if intersecting(iboid, newboid) {
                interoids.push(iboid.clone());
            }
        }
        for iboid in interoids {
            let partoids = take_outoids(&iboid, newboid);
            cuboids.retain(|x| *x != iboid);
            for p in partoids {
                cuboids.insert(p);
            }
        }
        if *on {
            cuboids.insert(newboid.clone());
        }
        println!("{:>2}/{:>2} {}", i, inp.len(), cuboids.len());

    }
    let mut acc = 0;
    for c in cuboids {
        acc += (c.x.count() * c.y.count() * c.z.count()) as u128
    }
    acc
}

fn main() {
    // let a =  Cuboid{x: 0..4, y: 0..4, z: 0..4};
    // let b =  Cuboid{x: 3..4, y: 3..4, z: 3..4};

    // let c = take_outoids(&a, &b);
    // println!("{:?}", c)


    let input = get_input("test.txt");
    println!("1: {}", solve1(&input));
    println!("2: {}", solve2(&input));
}
