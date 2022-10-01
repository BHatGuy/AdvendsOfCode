use std::fs;
use text_io::scan;

fn read_input(name: &str) -> Vec<(u32, u32, u32)> {
    let buf = fs::read_to_string(name).unwrap();
    let mut list = Vec::new();
    let lines = buf.trim_end().split("\n");
    for l in lines {
        let name: String;
        let speed: u32;
        let fly_time: u32;
        let pause_time: u32;

        scan!(l.bytes() => "{} can fly {} km/s for {} seconds, but then must rest for {} seconds.", name, speed, fly_time, pause_time);
        list.push((speed, fly_time, pause_time));
    }

    list
}

fn solve(reindeers: &Vec<(u32, u32, u32)>) {
    let mut winner = 0;
    for (speed, fly_t, pause_t) in reindeers {
        let mut pos = 0;
        let mut time = 0;

        while time <= 2503 {
            pos += fly_t * speed;
            time += fly_t;
            time += pause_t;
        }
        winner = winner.max(pos);
    }
    println!("{winner}");
}

#[derive(Clone)]
enum State {
    Resting(u32),
    Flying(u32),
}

fn solve2(reindeers: &Vec<(u32, u32, u32)>) {
    let mut scores = vec![0; reindeers.len()];
    let mut pos = vec![0; reindeers.len()];
    let mut states = vec![State::Flying(0); reindeers.len()];

    for _i in 0..2503 {
        for (i, (speed, fly_t, pause_t)) in reindeers.iter().enumerate() {
            match states[i] {
                State::Resting(mut x) => {
                    x += 1;
                    if x == *pause_t {
                        states[i] = State::Flying(0);
                    } else {
                        states[i] = State::Resting(x);
                    }
                }
                State::Flying(mut x) => {
                    pos[i] += speed;
                    x += 1;
                    if x == *fly_t {
                        states[i] = State::Resting(0);
                    } else {
                        states[i] = State::Flying(x);
                    }
                }
            }
        }
        let first_pos = pos.iter().max().unwrap();
        for (i, p) in pos.iter().enumerate() {
            if p == first_pos {
                scores[i] += 1;
            }
        }
    }
    let winner = scores.iter().max().unwrap();
    println!("{winner}")
}

fn main() {
    let input = read_input("input.txt");
    solve(&input);
    solve2(&input);
}
