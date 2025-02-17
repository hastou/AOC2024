use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

type Direction = (i32, i32);
const UP: Direction = (0, -1);
const DOWN: Direction = (0, 1);
const LEFT: Direction = (-1, 0);
const RIGHT: Direction = (1, 0);

#[derive(Eq, PartialEq, Hash, Clone)]
struct GameState {
    current_dir: Direction,
    current_position: (i32, i32),
}

fn game_step(
    game_state: GameState,
    map: &[Vec<char>],
) -> Option<GameState> {
    let (x, y) = game_state.current_position;
    if x < 0 || y < 0 || x >= map[0].len() as i32 || y >= map.len() as i32 {
        return None;
    }
    let mut next_pos = game_state.current_position;
    next_pos.0 += game_state.current_dir.0;
    next_pos.1 += game_state.current_dir.1;

    if next_pos.0 < 0
        || next_pos.1 < 0
        || next_pos.0 >= map[0].len() as i32
        || next_pos.1 >= map.len() as i32
    {
        return None;
    }

    if map[next_pos.1 as usize][next_pos.0 as usize] == '#' {
        let mut next_game_state = game_state.clone();
        match game_state.current_dir {
            UP => next_game_state.current_dir = RIGHT,
            DOWN => next_game_state.current_dir = LEFT,
            LEFT => next_game_state.current_dir = UP,
            RIGHT => next_game_state.current_dir = DOWN,
            _ => (),
        }
        return Some(next_game_state);
    }
    let mut next_game_state = game_state.clone();
    next_game_state.current_position = (x + game_state.current_dir.0, y + game_state.current_dir.1);

    Some(next_game_state)
}

fn is_game_state_a_loop(game_state: &GameState, map: &[Vec<char>]) -> bool {
    let mut game_state_history: std::collections::HashSet<GameState> =
        std::collections::HashSet::new();
    let mut game_state2 = game_state.clone();
    loop  {
        game_state_history.insert(game_state2.clone());
        match game_step(game_state2, &map) {
            Some(game_state3) => {game_state2 = game_state3;},
            None => break
        }
        if game_state_history.contains(&game_state2) {
            return true;
        }
    }
    false
}

fn get_initial_game_state(map: &[Vec<char>]) -> GameState {
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if let Some(direction) = match col {
                '>' => Some(RIGHT),
                '<' => Some(LEFT),
                '^' => Some(UP),
                'v' => Some(DOWN),
                _ => None,
            } {
                return GameState {
                    current_dir: direction,
                    current_position: (x as i32, y as i32),
                };
            }
        }
    }
    GameState {
        current_dir: UP,
        current_position: (0, 0),
    }
}

fn build_map(path: &Path) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut map: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        map.push(chars);
    }
    Ok(map)
}

fn main() -> io::Result<()> {
    let path = Path::new("ex6/input.txt");
    let mut map = build_map(&path)?;

    print_map(&map);

    let mut game_state = get_initial_game_state(&map);

    let mut position_history: std::collections::HashSet<(i32, i32)> =
        std::collections::HashSet::new();

    loop {
        position_history.insert(game_state.current_position);
        game_state = match game_step(game_state.clone(), &map) {
            Some(game_state) => game_state,
            None => break,
        };
    }
    println!("{:?}", position_history.len());

    game_state = get_initial_game_state(&map);
    position_history.clear();
    let mut options: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    println!(
        "game start : {}, {}",
        game_state.current_position.0, game_state.current_position.1
    );
    let start_pos = game_state.current_position.clone();

    use std::time::Instant;



    //
    // use rayon::ThreadPoolBuilder;
    // use rayon::prelude::*;
    //
    // let pool = ThreadPoolBuilder::new().num_threads(12).build().unwrap();
    // let options: std::sync::Mutex<std::collections::HashSet<(i32, i32)>> =
    //     std::sync::Mutex::new(std::collections::HashSet::new());
    // pool.install(|| {
    //     (0..map.len() as i32).into_par_iter().for_each(|y| {
    //         let mut map_clone = map.clone();
    //         let mut pos = 'a';
    //         for x in 0..map[0].len() as i32 {
    //             let current_position = (x, y);
    //             // println!("Checking position: {}, {}", x, y);
    //
    //             // let mut map_clone = map.clone();
    //             pos = map_clone[y as usize][x as usize];
    //             map_clone[y as usize][x as usize] = '#';
    //
    //
    //             if is_game_state_a_loop(&game_state, &map_clone) {
    //                 // println!("Loop found at position: {}, {}", x, y);
    //                 options.lock().unwrap().insert(current_position);
    //             }
    //             map_clone[y as usize][x as usize] = pos;
    //         }
    //     });
    // });
    // println!("{:?}", options.lock().unwrap().len());

    game_state = get_initial_game_state(&map);
    position_history.clear();
    let mut options: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    println!(
        "game start : {}, {}",
        game_state.current_position.0, game_state.current_position.1
    );
    let start_pos = game_state.current_position.clone();
    let initial_game_state = game_state.clone();
    let start_time = Instant::now();
    loop {
        let next_pos = (game_state.current_position.0 + game_state.current_dir.0, game_state.current_position.1 + game_state.current_dir.1);
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= map[0].len() as i32 || next_pos.1 >= map.len() as i32 || next_pos == start_pos {
            game_state = match game_step(game_state.clone(), &map) {
                Some(game_state) => game_state,
                None => break,
            };
            continue;
        }
        // let mut map = map.clone();
        let pos = map[next_pos.1 as usize][next_pos.0 as usize];
        map[next_pos.1 as usize][next_pos.0 as usize] = '#';
        // print_map_and_game_state(&map_clone, &game_state);
        if is_game_state_a_loop(&initial_game_state, &map) {
            // println!("OPTION");
            options.insert(next_pos);
        }
        map[next_pos.1 as usize][next_pos.0 as usize] = pos;
        game_state = match game_step(game_state.clone(), &map) {
            Some(game_state) => game_state,
            None => break,
        };
    }
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    // let start_pos = game_state.current_position;
    // while game_step(&mut game_state, &map) {
    //     let (x, y) = game_state.current_position;
    //     if x < 0
    //         || y < 0
    //         || x >= map[0].len() as i32
    //         || y >= map.len() as i32
    //         || map[y as usize][x as usize] == '#'
    //         || start_pos == game_state.current_position
    //     {
    //         previous_game_state = game_state.clone();
    //         continue;
    //     }
    //     // map[y as usize][x as usize] = 'O';
    //
    //     // print_map_and_game_state(&map, &game_state);
    //     //
    //     let mut map_clone = map.clone();
    //     map_clone[y as usize][x as usize] = '#';
    //
    //     let game_state_clone = previous_game_state.clone();
    //     previous_game_state = game_state.clone();
    //     print_map_and_game_state(&map_clone, &game_state_clone);
    //     println!("pos : {:?}", game_state_clone.current_position);
    //     println!("dir : {:?}", game_state_clone.current_dir);
    //     println!("obstacle pos : {:?}", game_state.current_position);
    //     if is_game_state_a_loop(&game_state_clone, &map_clone) {
    //         println!("OPTION");
    //         options.insert(game_state.current_position);
    //     }
    // }
    //
    // println!("Options positions: {:?}", options);
    println!("Options len: {:?}", options.len());

    // if options.contains(&start_pos) {
    //     println!("error {:?}", start_pos);
    // }

    Ok(())
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn print_map_and_game_state(map: &Vec<Vec<char>>, game_state: &GameState) {
    let mut map_clone = map.clone();

    // Update the current position in the map with the directional symbol
    map_clone[game_state.current_position.1 as usize][game_state.current_position.0 as usize] =
        match game_state.current_dir {
            UP => '^',
            DOWN => 'v',
            LEFT => '<',
            RIGHT => '>',
            _ => '#',
        };

    print_map(&map_clone);
}
// (3, 6)
// (6, 7)
// (7, 7)
// (1, 8)
// (3, 8)
// (7, 9)
// {(3, 8), (3, 6), (1, 8), (7, 7), (7, 9), (6, 7)}
