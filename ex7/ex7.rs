use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Equation {
    result: u128,
    values: Vec<u128>,
}

// Function to parse the file
fn parse_file(file_path: &str) -> io::Result<Vec<Equation>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut parsed_data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((key, values)) = line.split_once(":") {
            let result = key.trim().parse::<u128>().unwrap_or(0);
            let values = values
                .trim()
                .split_whitespace()
                .filter_map(|v| v.parse::<u128>().ok())
                .collect();

            parsed_data.push(Equation { result, values });
        }
    }

    Ok(parsed_data)
}

fn find_result(current: u128, values: &[u128], result: u128) -> bool {
    if current == result && values.is_empty() {
        return true;
    } else if values.is_empty() {
        return false;
    }

    find_result(current + values[0], &values[1..], result)
        || find_result(current * values[0], &values[1..], result)
        || find_result(
            format!("{}{}", current, values[0])
                .parse::<u128>()
                .unwrap(),
            &values[1..],
            result,
        )
}

fn main() {
    // Example usage
    let file_path = "ex7/input.txt"; // Replace with the actual file path
    let parsed_data = parse_file(file_path).unwrap();
    
    let mut total = 0;

    use std::time::Instant;

    let start_time = Instant::now();
    for data in parsed_data {
        if find_result(0, &data.values, data.result) {
            total += data.result;
        }
    }
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:.2?}", elapsed_time);
    println!("Total: {}", total);
}
