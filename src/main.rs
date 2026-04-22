use std::env;
use rubrix_cube_checker::io::{load_cube, parse_moves_file};
use rubrix_cube_checker::movements::apply_move;
use rubrix_cube_checker::compare::moves_to_solved;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run <cube_file> <moves_file>");
        std::process::exit(1);
    }

    let cube_file = &args[1];
    let moves_file = &args[2];

    // Load cube
    let cube = load_cube(cube_file);
    let mut new_cube = cube.clone();

    // Apply scramble moves
    let moves = parse_moves_file(moves_file);
    for m in moves {
        new_cube = apply_move(&new_cube, m.clone());
    }

    println!("Solved cube?: {:?}", new_cube.is_solved());

    // Solve
    match moves_to_solved(&new_cube) {
        Some(solution) => {
            println!("Solution: {:?}", solution);
        }
        None => {
            println!("No solution found (cube may be impossible or too deep).");
        }
    }
}