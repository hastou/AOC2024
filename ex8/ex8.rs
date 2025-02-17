use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn parse_file(file_path: &str) -> io::Result<(Vec<Vec<char>>)>{
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut parsed_data = Vec::new();
    for line in reader.lines() {
        let line = line?;
        parsed_data.push(line.chars().collect());
    }
    Ok(parsed_data)
}

fn main() {
    let file_path = "ex8/input.txt";
    let mut map = parse_file(file_path).unwrap();
    for row in map.iter() {
        println!("{:?}", row);
    }

    let mut hash_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col != &'.' {
                if !hash_map.contains_key(col) {
                    hash_map.insert(*col, Vec::new());
                }
                hash_map.get_mut(col).unwrap().push((x, y));
            }
        }
    }
    println!("{:?}", hash_map);

    let mut node_pos_set: HashSet<(isize, isize)> = HashSet::new();
    for (col, pos_list) in hash_map.iter() {
        for pos in pos_list {
            for pos2 in pos_list {
                if *pos != *pos2 {
                    let node_pos = (pos.0 as isize + (pos.0 as isize - pos2.0 as isize), pos.1 as isize + (pos.1 as isize - pos2.1 as isize));
                    if (node_pos.0 < 0 || node_pos.1 < 0 || node_pos.0 >= map[0].len() as isize || node_pos.1 >= map.len() as isize) {
                        continue;
                    }
                    node_pos_set.insert(node_pos);
                }
            }
        }
    }
    println!("{:?}", node_pos_set);
    println!("{:?}", node_pos_set.len());

    node_pos_set.clear();

    for (col, pos_list) in hash_map.iter() {
        for pos in pos_list {
            for pos2 in pos_list {
                if *pos != *pos2 {
                    let mut i = 0;
                    loop {
                        let node_pos = (pos.0 as isize + (pos.0 as isize - pos2.0 as isize) * i, pos.1 as isize + (pos.1 as isize - pos2.1 as isize) * i);
                        if (node_pos.0 < 0 || node_pos.1 < 0 || node_pos.0 >= map[0].len() as isize || node_pos.1 >= map.len() as isize) {
                            break   ;
                        }
                        i += 1;
                        node_pos_set.insert(node_pos);
                    }
                }
            }
        }
    }
    println!("{:?}", node_pos_set);
    println!("{:?}", node_pos_set.len());


}