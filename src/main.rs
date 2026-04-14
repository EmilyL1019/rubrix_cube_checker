use std::env;
use rubrix_cube_checker::io::{load_cube, parse_moves_file};
use rubrix_cube_checker::movements::apply_move;
use rubrix_cube_checker::compare::compare;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: cargo run <cube_file> <moves_file> <llm_cube_file>");
        std::process::exit(1);
    }

    let cube_file = &args[1];
    let moves_file = &args[2];
    let llm_cube_file = &args[3];

    // Load cubes
    let mut cube = load_cube(cube_file);
    let llm_cube = load_cube(llm_cube_file);

    // Load moves
    let moves = parse_moves_file(moves_file);

    // Apply moves
    for m in moves {
        cube = apply_move(cube, m);
    }

    // Compare
    let diff = compare(&cube, &llm_cube);

    if diff == 0 {
        println!("Correct.");
    } else {
        println!("Incorrect with {} incorrect cells.", diff);
    }
}