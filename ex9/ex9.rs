use std::io::Read;

fn find_first_space(line: &Vec<Option<u32>>, minimal_space_size: u32) -> Option<usize> {
    let mut current_space = 0;
    for (i, c) in line.iter().enumerate() {
        if c == &None {
            current_space += 1;
        } else {
            current_space = 0;
        }
        if current_space == minimal_space_size {
            return Some(i - current_space as usize + 1);
        }
    }
    None
}

fn print_line(line: &Vec<Option<u32>>) {
    for c in line.iter() {
        match c {
            Some(x) => print!("{}", x),
            None => print!("."),
        }
    }
    println!();
}

fn main() {
    let file_path = "ex9/input.txt";
    let mut file = std::fs::File::open(file_path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut line: Vec<Option<u32>> = Vec::new();
    let mut id = 0;
    for (i, c) in buffer.chars().enumerate() {
        if i % 2 == 0 {
            for d in 0..c.to_digit(10).unwrap() {
                line.push(Some(id));
            }
            id += 1;
        } else {
            for d in 0..c.to_digit(10).unwrap() {
                line.push(None);
            }
        }
    }
    println!("{}", buffer);
    println!("{:?}", line);
    let mut line_copy = line.clone();
    let mut start = 0;
    let mut end = line.len() - 1;
    while start < end {
        while line[start] != None {
            start += 1;
        }
        while line[end] == None {
            end -= 1;
        }
        line.swap(start, end);
    }

    if line[start] != None && line[end] == None {
        line.swap(start, end);
    }
    print_line(&line_copy);
    checksum(&mut line);
    let mut current_id: Option<u32> = line_copy[line.len() - 1];
    let mut current_block_size: u32 = 0;
    let mut current_index = line_copy.len() - 1;
    loop {
        if line_copy[current_index] == current_id {
            current_block_size += 1;
        } else if current_id != line_copy[current_index]
        {
            if current_id != None {
                let space_index = find_first_space(&line_copy, current_block_size);
                let id = line_copy[current_index + 1];
                if space_index.is_some_and(|x| x < current_index) {
                    for i in space_index.unwrap()..space_index.unwrap() + current_block_size as usize {
                        line_copy[i] = id;
                    }
                    for i in current_index+1..current_index+1 + current_block_size as usize {
                        line_copy[i] = None;
                    }
                }
            }
            current_id = line_copy[current_index];
            current_block_size = 1;
        }
        if current_index == 0 {
            break;
        }
        current_index -= 1;
    }
    print_line(&line_copy);
    print!("{}", checksum(&mut line_copy));
}

fn checksum(line: &mut Vec<Option<u32>>) -> usize {
    let mut total: usize = 0;
    for (i, c) in line.iter().enumerate() {
        if c != &None {
            total += i * c.unwrap() as usize;
        }
    }
    total
}
