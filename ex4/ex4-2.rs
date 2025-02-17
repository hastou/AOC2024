use std::fs::File;
use std::path::Path;
type Grid = Vec<Vec<char>>;
use std::io::{self, BufRead};
trait GridUtilities {
    fn print(&self);
}
impl GridUtilities for Grid {
    fn print(&self) {
        for row in self {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    let mut grid: Grid = Vec::new();

    if let Ok(lines) = read_lines("ex4/input.txt") {
        for line in lines {
            if let Ok(content) = line {
                grid.push(content.chars().collect());
            }
        }
    }


    fn find_word(grid: &Grid, word: &str, row: usize, col: usize, direction: (isize, isize), index: usize) -> bool {
        if index == word.len() {
            return true;
        }
        let (dr, dc) = direction;
        let new_row = row.wrapping_add(dr as usize);
        let new_col = col.wrapping_add(dc as usize);

        if new_row >= grid.len() || new_col >= grid[new_row].len() || grid[new_row][new_col] != word.chars().nth(index).unwrap() {
            return false;
        }

        find_word(grid, word, new_row, new_col, direction, index + 1)
    }

    let directions = vec![
        (1, 1),   // Diagonal Down-Right
        (1, -1),  // Diagonal Down-Left
        (-1, 1),  // Diagonal Up-Right
        (-1, -1), // Diagonal Up-Left
    ];

    let xmas_word = "MAS";
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    println!("Finding word: {}", xmas_word);

    let mut mas_centers: Vec<(u32, u32)> = vec![];

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == xmas_word.chars().next().unwrap() {
                for &direction in &directions {
                    if find_word(&grid, xmas_word, row, col, direction, 1) {
                        println!(
                            "Found word {} starting at ({}, {}) in direction {:?}",
                            xmas_word, row, col, direction
                        );
                        mas_centers.push(((row as isize + direction.0) as u32, (col as isize + direction.1) as u32));
                    }
                }
            }
        }
    }


    use std::collections::HashMap;

    let mut center_counts: HashMap<(u32, u32), u32> = HashMap::new();

    for &center in &mas_centers {
        *center_counts.entry(center).or_insert(0) += 1;
    }

    let duplicates: Vec<((u32, u32), u32)> = center_counts
        .iter()
        .filter(|&(_, &count)| count > 1)
        .map(|(&center, &count)| (center, count))
        .collect();

    println!("Duplicate centers and their counts:");
    for ((row, col), count) in duplicates.clone() {
        println!("Center at ({}, {}) appears {} times", row, col, count);
    }
    println!("Found {} duplicates", duplicates.len());
}
