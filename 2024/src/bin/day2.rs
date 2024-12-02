use std::fs;

fn read_reports(name: &str) -> Vec<Vec<u64>> {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut reports = Vec::new();
    for l in lines {
        let mut report = Vec::new();
        for level in l.split(" ") {
            let level = str::parse(level).expect("Could not parse level");
            report.push(level);
        }
        reports.push(report);
    }

    reports
}

fn check_safety(report: &Vec<u64>) -> bool {
    let mut last = report[0];
    let mut order = std::cmp::Ordering::Equal;
    for level in &report[1..] {
        let diff = i128::from(last) - i128::from(*level);
        if diff < 0 {
            if order == std::cmp::Ordering::Greater {
                return false;
            }
            order = std::cmp::Ordering::Less;
        } else if diff > 0 {
            if order == std::cmp::Ordering::Less {
                return false;
            }
            order = std::cmp::Ordering::Greater;
        } else {
            return false;
        }
        if diff.abs() > 3 {
            return false;
        }
        last = *level;
    }
    return true;
}

fn main() {
    let reports = read_reports("input2.txt");

    let mut safe_count = 0;
    for report in &reports {
        if check_safety(report) {
            safe_count += 1;
        }
    }
    println!("{safe_count}");

    let mut safe_count = 0;
    for report in &reports {
        if check_safety(report) {
            safe_count += 1;
        } else {
            for index in 0..report.len() {
                let mut new_report = report.to_vec();
                new_report.remove(index);
                if check_safety(&new_report) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    println!("{safe_count}");
}
