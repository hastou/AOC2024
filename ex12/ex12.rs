use std::collections::HashSet;
use std::ops::Add;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point(tuple.0, tuple.1)
    }
}

fn main() {
    let filepath = "ex12/example.txt";
    let contents = std::fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let mut map: Vec<Vec<char>> = vec![];
    for (i, line) in contents.lines().enumerate() {
        map.push(vec![]);
        for c in line.chars() {
            map[i].push(c);
        }
    }
    print_map(&mut map);

    let mut visited_pos: HashSet<Point> = HashSet::new();
    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if visited_pos.contains(&Point(j as i32, i as i32)) {
                continue;
            }
            let (area, perimeter) = compute_region(Point(j as i32, i as i32), &map, &mut visited_pos);
            let side_count = compute_region_side_count(Point(j as i32, i as i32), &map, &mut visited_pos);

            println!("Region {}: area = {}, perimeter = {}, sides = {}", map[j][i], area, perimeter, side_count);
            total += area * perimeter;
        }
    }
    println!("Total: {}", total);
}

fn compute_region_side_count(pos: Point, map: &Vec<Vec<char>>, visited_pos: &mut HashSet<Point>) -> i32 {
    let mut side_count = 0;
    let mut pos = pos.clone();
    let region: char = map[pos.1 as usize][pos.0 as usize];
    let directions: Vec<Point> = vec![
        Point::from((0, 1)), // DOWN
        Point::from((-1, 0)), // LEFT
        Point::from((0, -1)), // UP
        Point::from((1, 0)), // RIGHT
    ];
    let mut next_pos ;
    loop {
        // if
        next_pos = pos + directions[0];
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.1 >= map.len() as i32 || next_pos.0 >= map[0].len() as i32 || map[next_pos.1 as usize][next_pos.0 as usize] != region {
            break;
        }
        pos = next_pos;
    }

    loop {
        next_pos = pos + directions[3];
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.1 >= map.len() as i32 || next_pos.0 >= map[0].len() as i32 || map[next_pos.1 as usize][next_pos.0 as usize] != region {
            break;
        }
        pos = next_pos;
    }

    let start_pos = pos.clone();
    let start_dir = 2;
    let mut current_dir = start_dir;
    loop {
        let next_pos = pos + directions[current_dir];
        print_map_pos(&map, pos, Some(next_pos));

        use std::io;
        print!("{:?}, {:?}", start_pos, next_pos);
        println!("Press Enter to continue...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        if next_pos == start_pos && current_dir == start_dir {
            side_count += 1;
            break;
        }
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.1 >= map.len() as i32 || next_pos.0 >= map[0].len() as i32 || map[next_pos.1 as usize][next_pos.0 as usize] != region {
            current_dir = (current_dir+
                1)%4;
            side_count += 1;
            continue;
        } else {
            pos = next_pos;
        }
    }

    side_count
}

fn compute_region(pos: Point, map: &Vec<Vec<char>>, visited_pos: &mut HashSet<Point>) -> (usize, usize) {
    let mut stack: Vec<Point> = vec![pos];
    let mut area = 0;
    let mut perimeter = 0;
    let region: char = map[pos.1 as usize][pos.0 as usize];
    let directions: Vec<Point> = vec![
        Point::from((0, 1)),
        Point::from((1, 0)),
        Point::from((0, -1)),
        Point::from((-1, 0)),
    ];
    loop {
        if stack.len() == 0 {
            break;
        }
        let mut actual_pos = stack.pop().unwrap();

        if visited_pos.contains(&actual_pos) {
            continue;
        }
        if map[actual_pos.1 as usize][actual_pos.0 as usize] == region {
            area += 1;
        }
        visited_pos.insert(actual_pos);
        for direction in &directions {
            let neighbor_pos = actual_pos + *direction;
            if neighbor_pos.0 < 0 || neighbor_pos.1 < 0 || neighbor_pos.0 >= map.len() as i32 || neighbor_pos.1 >= map[0].len() as i32 || map[neighbor_pos.1 as usize][neighbor_pos.0 as usize] != region {
                perimeter += 1;
            } else {
                stack.push(neighbor_pos);
            }
        }
    }
    (area, perimeter)
}

fn print_map(map: &mut Vec<Vec<char>>) {
    for row in map.iter() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}
fn print_map_pos(map: &Vec<Vec<char>>, pos: Point, next_pos: Option<Point>) {
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if next_pos.is_some() && next_pos.unwrap() == Point(j as i32, i as i32) {
                print!("X");
                continue;
            }
            if i == pos.1 as usize && j == pos.0 as usize {
                print!("*");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!();
}