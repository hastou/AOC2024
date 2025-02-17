use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("/Users/quentin/RustroverProjects/advent2024/ex1/input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let (mut column1, mut column2): (Vec<usize>, Vec<usize>) = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let mut numbers = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok());
            Some((numbers.next()?, numbers.next()?))
        })
        .unzip();

    println!("Column 1: {:?}", column1);
    println!("Column 2: {:?}", column2);
    for column in [&mut column1, &mut column2] {
        column.sort();
    }

    let mut frequency = HashMap::new();
    frequency.insert(2, 3);
    for i in column1.clone() {
        frequency.insert(i, column2.iter().filter(|&&x| x == i).count());
    }

    let mut total_distance = 0;
    for j in column1 {
        total_distance += j * frequency.get(&j).unwrap();
    }
    println!("Total distance: {}", total_distance);
    Ok(())
}
