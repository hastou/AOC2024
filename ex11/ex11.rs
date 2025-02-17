use std::collections::HashMap;
use std::io::{BufRead, Read};

fn compute_stone_count(stone: u128, num_rounds: u128, cache: &mut HashMap<(u128, u128), u128>) -> u128 {
    if num_rounds == 0 {
        return 1;
    }
    let cache_key = (stone, num_rounds);
    if cache.contains_key(&cache_key) {
        return cache[&cache_key];
    }
    let mut result = 0;
    if stone == 0 {
        result = compute_stone_count(1, num_rounds - 1, cache)
    } else if stone.to_string().len() % 2 == 0 {
        let middle = u128::div_euclid(stone.to_string().len() as u128, 2) as usize;
        let left = stone.to_string()[0..middle].parse().unwrap();
        let right = stone.to_string()[middle..].parse().unwrap();
        result = compute_stone_count(left, num_rounds - 1, cache) + compute_stone_count(right, num_rounds - 1, cache)
    } else {
        result = compute_stone_count(stone*2024, num_rounds - 1, cache)
    }
    cache.insert(cache_key, result);
    result
}

fn main() {
    let file_path = "ex11/input.txt";
    let mut file = std::fs::File::open(file_path).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    let mut vec = file_content
        .split(' ')
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    println!("{:?}", vec);

    let mut total = 0;
    for stone in vec {
        total += compute_stone_count(stone, 75, &mut HashMap::new());
    }
    println!("{}", total);
}
