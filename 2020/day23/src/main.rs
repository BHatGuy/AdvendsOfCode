use std::char;
use std::cmp;
use std::rc::Rc;

#[derive(Debug)]
struct Cup {
    lable: u64,
    next: Option<Rc<Cup>>,
}

#[derive(Debug)]
struct Circle {
    circle: Rc<Cup>,
    pickedup: Option<Rc<Cup>>,
    current: Rc<Cup>,
    destination: Rc<Cup>,
    min: u64,
    max: u64,
}

impl Cup {
    fn new(lable: u64) -> Self {
        Self { lable, next: None }
    }
}

impl Circle {
    fn new(inp: &[u64], len: usize) -> Self {
        let first = Rc::new(Cup::new(inp[0]));
        let mut prev = first;
        let mut max = u64::MIN;
        let mut min = u64::MAX;
        for l in inp.iter().skip(1) {
            let c = Rc::new(Cup::new(*l));
            prev.next = Some(c);
            prev = c;

            max = cmp::max(max, *l);
            min = cmp::min(min, *l);
        }

        let mut next = max + 1;
        for _ in 0..len - inp.len() {
            let c = Rc::new(Cup::new(next));
            prev.next = Some(c);
            prev = c;
            next += 1;
            max = cmp::max(max, next);
            min = cmp::min(min, next);
        }
        prev.next = Some(first);
        Self {
            circle: first,
            pickedup: None,
            current: first.clone(),
            destination: first.clone(),
            max,
            min,
        }
    }

    fn pick_up(&mut self) {
        self.pickedup = self.current.next;
        let mut last = self.pickedup.unwrap();
        last = last.next.unwrap();
        last = last.next.unwrap();
        self.current.next = last.next;
        last.next = None;
        let n = self.pickedup;
        while let Some(c) = n {
            self.max = cmp::max(self.max, c.lable);
            self.min = cmp::min(self.min, c.lable);
            n = c.next;
        }
    }

    fn select_destination(&mut self) {
        let mut new_dest = self.current.lable;
        'outer: while new_dest >= self.min {
            let n = self.pickedup;
            while let Some(c) = n {
                new_dest -= 1;
                if c.lable == new_dest {
                    continue 'outer;
                }
                n = c.next;
            }

            break;
        }
        if new_dest < self.min {
            new_dest = self.max;
        }
        let c = self.current;

        loop {
            new_dest -= 1;
            if c.lable == new_dest {
                self.destination = c.clone();
                return;
            }
            c = c.next.unwrap();
        }
    }

    fn place(&mut self) {
        let last = self.pickedup.unwrap();
        loop {
            if let Some(c) = last.next {
                last = c;
            } else {
                break;
            }
        }
        last.next = Some(self.destination.next.unwrap().clone());
        self.destination.next = Some(last.clone());
        self.pickedup = None;
    }

    fn select_current(&mut self) {
        self.current = self.current.next.unwrap();
    }

    fn play(&mut self, n: u64) {
        for i in 0..n {
            self.pick_up();
            self.select_destination();
            self.place();
            self.select_current();
            let pro = ((i + 1) as f32 / n as f32) * 100f32;
            print!("\r{:08}/{:08} ({:05.2}%)          ", i + 1, n, pro);
        }
        println!("");
    }

    fn get_solution1(&self) -> String {
        let mut s = String::new();
        let one = self.current;
        loop {
            if one.lable == 1 {
                break;
            }
            if let Some(n) = one.next {
                one = n;
            }
        }
        loop {
            if let Some(n) = one.next {
                one = n;
            }
            if one.lable == 1 {
                break;
            }
            s.push(char::from_digit(one.lable as u32, 10).unwrap());
        }
        s
    }

    fn get_solution2(&self) -> String {
        let mut one = self.current.clone();
        loop {
            if one.lable == 1 {
                break;
            }
            if let Some(n) = one.next.as_ref() {
                one = n.clone();
            }
        }
        let n1 = one.next.as_ref().unwrap();
        let n2 = n1.next.as_ref().unwrap();
        let val =  n1.lable * n2.lable;

        val.to_string()
    }
}

static INPUT: &[u64] = &[1, 5, 7, 6, 2, 3, 9, 8, 4];
static TEST_INPUT: &[u8] = &[3, 8, 9, 1, 2, 5, 4, 6, 7];

fn main() {
    let mut c = Circle::new(INPUT, INPUT.len());
    c.play(100);
    println!("{}", c.get_solution1());

    let mut c = Circle::new(INPUT, 1000000);
    c.play(10000000);
    println!("{}", c.get_solution2());
}
