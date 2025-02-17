use std::collections::HashMap;
use regex::Regex;
use std::fs;

enum Token {
    Mul(u32, u32),
    Do,
    Donot,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "ex3/input.txt";
    let content = fs::read_to_string(file_path)?;

    let mul_regex = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut tokens: HashMap<usize, Token> = HashMap::new();
    for do_token in do_regex.find_iter(&content) {
        tokens.insert(do_token.start(), Token::Do);
    }
    for dont_token in dont_regex.find_iter(&content) {
        tokens.insert(dont_token.start(), Token::Donot);
    }
    for mul_token in mul_regex.captures_iter(&content) {
        let x: u32 = mul_token[1].parse().unwrap();
        let y: u32 = mul_token[2].parse().unwrap();
        let start = mul_token.get(0).unwrap().start();
        tokens.insert(start, Token::Mul(x, y));
    }

    let mut positions_sorted: Vec<_> = tokens.keys().cloned().collect();
    positions_sorted.sort();

    let mut do_state= true;
    let mut total: u32 = 0;

    for position in positions_sorted {
        match tokens.get(&position) {
            Some(Token::Mul(x, y))  => {
                if do_state {
                    total += x * y;
                }
            },
            Some(Token::Do) => {
                do_state = true;
            },
            Some(Token::Donot) => {
                do_state = false;
            }
            _ => println!("unknown"),
        }
    }

    println!("result = {}", total);

    Ok(())
}