use std::collections::HashSet;
use std::io::Read;


fn compute_trailhead_score(pos: (i32, i32), map: &Vec<Vec<i32>>) -> HashSet<(i32, i32)> {
    let mut stack = vec![pos];
    let mut visited = HashSet::new();
    let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];

    while let Some(current) = stack.pop() {
        if map[current.1 as usize][current.0 as usize] == 9 {
            visited.insert(current);
        }

        for direction in &directions {
            let new_pos = (current.0 + direction.0, current.1 + direction.1);
            if new_pos.0 >= 0 && new_pos.1 >= 0 && new_pos.0 < map.len() as i32 && new_pos.1 < map[0].len() as i32 {
                if map[current.1 as usize][current.0 as usize] + 1 == map[new_pos.1 as usize][new_pos.0 as usize] {
                    stack.push(new_pos);
                }
            }
        }
    }
    visited
}
fn compute_trailhead_score2(pos: (i32, i32), map: &Vec<Vec<i32>>) -> u32 {
    if map[pos.1 as usize][pos.0 as usize] == 9 {
        return 1;
    }
    let directions: Vec<(i32,i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut total = 0;
    for direction in directions.iter() {
        let new_pos = (pos.0 + direction.0, pos.1 + direction.1);
        if new_pos.0 >= 0 && new_pos.1 >= 0 && new_pos.0 < map.len() as i32 && new_pos.1 < map[0].len() as i32 {
            if map[pos.1 as usize][pos.0 as usize] + 1 == map[new_pos.1 as usize][new_pos.0 as usize] {
                total += compute_trailhead_score2(new_pos, &map);
            }
        }
    }
    total
}

fn compute_trailhead_score2_iter(pos: (i32, i32), map: &Vec<Vec<i32>>) -> u32 {
    let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];

    struct Frame {
        pos: (i32, i32),
        children: Vec<(i32, i32)>,
        index: usize,
        partial: u32,
    }

    fn compute_children(pos: (i32, i32), map: &Vec<Vec<i32>>, dirs: &[(i32, i32)]) -> Vec<(i32, i32)> {
        let mut children = Vec::new();
        let current_value = map[pos.1 as usize][pos.0 as usize];
        for d in dirs {
            let new_pos = (pos.0 + d.0, pos.1 + d.1);
            if new_pos.0 >= 0 && new_pos.1 >= 0 &&
                new_pos.0 < map.len() as i32 && new_pos.1 < map[0].len() as i32
            {
                if current_value + 1 == map[new_pos.1 as usize][new_pos.0 as usize] {
                    children.push(new_pos);
                }
            }
        }
        children
    }

    let initial_children =
        if map[pos.1 as usize][pos.0 as usize] != 9 {
            compute_children(pos, map, &directions)
        } else {
            Vec::new()
        };
    let mut stack = Vec::new();
    stack.push(Frame { pos, children: initial_children, index: 0, partial: 0 });

    let mut result = 0;

    while let Some(frame) = stack.last_mut() {
        let current_value = map[frame.pos.1 as usize][frame.pos.0 as usize];

        if current_value == 9 {
            stack.pop();
            let branch_result = 1;
            if let Some(parent) = stack.last_mut() {
                parent.partial += branch_result;
            } else {
                result = branch_result;
                break;
            }
        } else if frame.index < frame.children.len() {
            let child_pos = frame.children[frame.index];
            frame.index += 1;
            let child_value = map[child_pos.1 as usize][child_pos.0 as usize];
            let child_children = if child_value != 9 {
                compute_children(child_pos, map, &directions)
            } else {
                Vec::new()
            };
            stack.push(Frame { pos: child_pos, children: child_children, index: 0, partial: 0 });
        } else {
            let completed_frame = stack.pop().unwrap();
            let branch_result = completed_frame.partial;
            if let Some(parent) = stack.last_mut() {
                parent.partial += branch_result;
            } else {
                result = branch_result;
                break;
            }
        }
    }
    result
}



fn main() {
    let path = "ex10/input.txt";
    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let mut map: Vec<Vec<i32>> = Vec::new();
    for (y, line) in buffer.lines().enumerate() {
        map.push(vec![0; line.len()]);
        for (x, c) in line.chars().enumerate() {
            map[y][x] = c.to_digit(10).unwrap() as i32;
        }
    }
    println!("{:#?}", map);
    let mut total = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                let score = compute_trailhead_score((x as i32, y as i32), &map).len();
                total += score;
            }
        }
    }
    println!("{}", total);
    let mut total = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                let score = compute_trailhead_score2((x as i32, y as i32), &map);
                total += score;
            }
        }
    }
    println!("{}", total);
}