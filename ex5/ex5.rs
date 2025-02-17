use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_file(file_path: &str) -> Result<(Vec<(i32, i32)>, Vec<Vec<i32>>), io::Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut pairs = Vec::new();
    let mut matrix = Vec::new();

    let mut is_second_part = false;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            is_second_part = true;
            continue;
        }
        if !is_second_part {
            let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                if let (Ok(first), Ok(second)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                {
                    pairs.push((first, second));
                }
            }
        } else {
            let row: Vec<i32> = line
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            matrix.push(row);
        }
    }

    Ok((pairs, matrix))
}

fn is_row_valid(row: &Vec<i32>, pairs: &Vec<(i32, i32)>) -> bool {
    for rule in pairs {
        if let (Some(index_0), Some(index_1)) = (
            row.iter().position(|&x| x == rule.0),
            row.iter().position(|&x| x == rule.1),
        ) {
            if index_0 > index_1 {
                return false;
            }
        }
    }
    true
}
fn main() {
    use std::time::Instant;

    let start_time = Instant::now();

    let result = parse_file("ex5/input.txt").expect("Failed to parse file");

    let (pairs, matrix) = result;
    println!("{:?}", pairs);
    println!("{:?}", matrix);
    
    let mut invalid_rows: Vec<Vec<i32>> = vec![];

    let mut result = 0;
    for row in &matrix {
        println!("{:?}", row);
        let row_valid = is_row_valid(row, &pairs);
        if row_valid {
            result += row.get(row.len() / 2).unwrap();
        } else {
            invalid_rows.push(row.clone());
        }
    }
    println!("part 1 result = {:?}", result);

    let elapsed_time = start_time.elapsed();
    println!("part 1 Execution time: {:?}", elapsed_time);
    result = 0;
    for mut row in invalid_rows {
        while !is_row_valid(&row, &pairs) {
            for rule in &pairs {
                if let (Some(index_0), Some(index_1)) = (
                    row.iter().position(|&x| x == rule.0),
                    row.iter().position(|&x| x == rule.1),
                ) {
                    if index_0 > index_1 {
                        let value_to_move = row[index_0];
                        let mut updated_row = row.clone();
                        updated_row.remove(index_0);
                        updated_row.insert(index_1, value_to_move);
                        row = updated_row;
                    }
                }
            }
        }
        result += row.get(row.len() / 2).unwrap();
    }
    println!("part 2 result = {:?}", result);
    let elapsed_time = start_time.elapsed();
    println!("part 2 Execution time: {:?}", elapsed_time);
}
