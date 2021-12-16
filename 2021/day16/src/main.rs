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

fn print_bits(bits: &[bool]) {
    for &b in bits {
        print!("{}", if b {1} else {0});
    }
    println!();
}

fn as_u16(bits: &[bool]) -> u16 {
    if bits.len() > 16 {
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

fn parse_literal(bits: &[bool]) -> usize {
    let mut sum = 0;
    let mut length = 0;
    for group in bits.chunks(5) {
        length += 5;
        sum = (sum << 4) + as_u16(&group[1..]);
        if !group[0] {
            break;
        }
    }
    length
}

fn parse_operator(bits: &[bool], versions: &mut Vec<u16>) -> usize {
    let length_id = bits[0];
    let bits = &bits[1..];
    if !length_id {
        let length = as_u16(&bits[0..15]) as usize;
        let mut rest = &bits[15..];
        let mut len = length;
        while len > 0 {
            let p = parse_packet(rest, versions);
            len -= p;
            rest = &rest[p..];
        }
        return length + 15 + 1;
    } else {
        let mut length = 0;
        let count = as_u16(&bits[0..11]);
        let mut rest = &bits[11..];
        for _ in 0..count {
            let p = parse_packet(rest, versions);
            length += p;
            rest = &rest[p..];
        }
        return length + 11 + 1;
    }
}

fn parse_packet(bits: &[bool], versions: &mut Vec<u16>) -> usize {
    print_bits(bits);
    let version = &bits[0..3];
    versions.push(as_u16(version));
    let typ = &bits[3..6];
    let rest = &bits[6..];
    let length = match as_u16(typ) {
        4 => parse_literal(rest),
        _ => parse_operator(rest, versions),
    };

    length + 3 + 3
}

fn solve1(inp: &InputType) -> u16 {
    let mut versions = Vec::new();
    parse_packet(inp, &mut versions);
    versions.iter().sum()
}

// fn solve2(_: &InputType) -> u64 {
//     0
// }

fn main() {
    let input = get_input("input.txt");
    for inp in input.iter() {
        println!("1: {}", solve1(&inp));
        // println!("2: {}", solve2(&inp));
    }
}
