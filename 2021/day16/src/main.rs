use std::fs;

type InputType = Vec<bool>;

fn get_input(name: &str) -> Vec<InputType> {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut input = Vec::new();

    for l in lines {
        let mut bits = Vec::new();
        for c in l.trim().chars() {
            let val: u8 = c.to_digit(16).unwrap().try_into().unwrap();
            for i in (0..4).rev() {
                let bit = val >> i & 1;
                bits.push(bit == 1);
            }
        }
        input.push(bits);
    }
    input
}

fn _print_bits(bits: &[bool]) {
    for &b in bits {
        print!("{}", if b { 1 } else { 0 });
    }
    println!();
}

fn as_int(bits: &[bool]) -> u64 {
    if bits.len() > 64 {
        panic!("too long: {:?}", bits);
    }
    let mut byte = 0;
    for (i, &bit) in bits.iter().rev().enumerate() {
        if bit {
            byte |= 1 << i;
        }
    }
    byte
}

fn parse_literal(bits: &[bool]) -> (u64, usize) {
    let mut sum = 0;
    let mut length = 0;
    for group in bits.chunks(5) {
        length += 5;
        sum = (sum << 4) + as_int(&group[1..]);
        if !group[0] {
            break;
        }
    }
    (sum.into(), length)
}

fn parse_operator(bits: &[bool], op: u64, versions: &mut Vec<u64>) -> (u64, usize) {
    let length_id = bits[0];
    let bits = &bits[1..];
    let mut vals = Vec::new();
    let mut length;
    if !length_id {
        length = as_int(&bits[0..15]) as usize;
        let mut rest = &bits[15..];
        let mut len = length;
        while len > 0 {
            let (val, p_len) = parse_packet(rest, versions);
            vals.push(val);
            len -= p_len;
            rest = &rest[p_len..];
        }
        length = length + 15 + 1;
    } else {
        length = 0;
        let count = as_int(&bits[0..11]);
        let mut rest = &bits[11..];
        for _ in 0..count {
            let (val, p_len) = parse_packet(rest, versions);
            vals.push(val);
            length += p_len;
            rest = &rest[p_len..];
        }
        length = length + 11 + 1;
    }
    let res = match op {
        0 => vals.iter().sum(),
        1 => vals.iter().product(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => if vals[0] > vals[1] {1} else {0},
        6 => if vals[0] < vals[1] {1} else {0},
        7 => if vals[0] == vals[1] {1} else {0},
        x => panic!("Unknown opcode {}", x), 
    };

    (res, length)
}

fn parse_packet(bits: &[bool], versions: &mut Vec<u64>) -> (u64, usize) {
    let version = &bits[0..3];
    versions.push(as_int(version));
    let typ = &bits[3..6];
    let rest = &bits[6..];
    
    let op = as_int(typ);
    let (val, length) = if op == 4 {
        parse_literal(rest)
    } else {
        parse_operator(rest, op, versions)
    };

    (val, length + 3 + 3)
}

fn solve1(inp: &InputType) -> u64 {
    let mut versions = Vec::new();
    parse_packet(inp, &mut versions);
    versions.iter().sum()
}

fn solve2(inp: &InputType) -> u64 {
    let mut versions = Vec::new();
    let res = parse_packet(inp, &mut versions);
    res.0
}

fn main() {
    let input = get_input("input.txt");
    for inp in input.iter() {
        println!("1: {}", solve1(&inp));
        println!("2: {}", solve2(&inp));
        println!();
    }
}
