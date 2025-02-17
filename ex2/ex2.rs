use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::Direction::{Ascending, Descending, Unknown};

type Report = Vec<i32>;
#[derive(PartialEq)]
enum Direction {
    Ascending,
    Descending,
    Unknown
}

fn is_report_safe(report: Report, recursive: bool) -> bool {
    let mut direction = Unknown;
    for (i, _) in report.iter().enumerate() {
        if i == report.len() - 1 {
            break;
        }
        let mut error = false;
        let diff = (report[i] - report[i + 1]).abs();
        if report[i] > report[i + 1] && [Unknown, Descending].contains(&direction) {
            direction = Descending;
        } else if report[i] < report[i + 1] && [Unknown, Ascending].contains(&direction) {
            direction = Ascending;
        } else {
            error = true;
        }
        if diff > 3 || diff < 1 || error {
            if !recursive {
                return false;
            }
            let mut report_copy = report.clone();
            let mut report_copy2 = report.clone();
            report_copy.remove(i);
            report_copy2.remove(i + 1);
            return is_report_safe(report_copy, false) || is_report_safe(report_copy2, false);
        }
    }
    true
}

fn main() -> io::Result<()> {
    let path = Path::new("/Users/quentin/RustroverProjects/advent2024/ex2/input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut report_list: Vec<Report> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Report = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        report_list.push(numbers);
    }

    let mut safe_report_count = 0;

    for report in report_list {
        if is_report_safe(report, true) {
            safe_report_count += 1;
        }
    }
    println!("safe report count is {}", safe_report_count);

    Ok(())
}