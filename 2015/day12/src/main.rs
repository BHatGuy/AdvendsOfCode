use json::{self, JsonValue};
use std::io;

fn get_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Cant read input");

    buf
}

fn sum(obj: &JsonValue, ignore_red: bool) -> f64 {
    match obj {
        JsonValue::Null | JsonValue::String(_) | JsonValue::Boolean(_) | JsonValue::Short(_) => {
            0f64
        }
        JsonValue::Number(x) => (*x).into(),
        JsonValue::Object(o) => {
            let mut res = 0f64;
            for (_, val) in o.iter() {
                if ignore_red {
                    match val {
                        JsonValue::String(s) => {
                            if s == "red" {
                                res = 0f64;
                                break;
                            }
                        }
                        JsonValue::Short(s) => {
                            if s == "red" {
                                res = 0f64;
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                res += sum(val, ignore_red);
            }
            res
        }
        JsonValue::Array(a) => {
            let mut res = 0f64;
            for e in a {
                res += sum(e, ignore_red);
            }
            res
        }
    }
}

fn solve(input: &String) {
    let obj = json::parse(input);
    if let Ok(obj) = obj {
        let res1 = sum(&obj, false);
        let res2 = sum(&obj, true);
        println!("{res1} {res2}");
    }
}

fn main() {
    let input = get_input();
    solve(&input);
}
