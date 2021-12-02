const INPUT: &(u64, u64) = &(1965712, 19072108);
const P: u64 = 20201227;


fn find_loopsize(pubkey: u64) -> u64 {
    let m = 7;
    let mut val = 1;
    let mut loopsize = 0;
    while val != pubkey {
        val = val * m % P;
        loopsize += 1;
    }
    loopsize
}

fn transform(m: u64, loopsize: u64) -> u64{
    let mut val = 1;
    for _ in 0..loopsize {
        val = val * m % P;
    }
    val
}

fn solve1() -> u64 {
    let &(pubkey1, pubkey2) = INPUT;
    let loopsize = find_loopsize(pubkey1);
    let privkey = transform(pubkey2, loopsize);
    privkey
}

fn main() {
    println!("1: {}", solve1());
}
