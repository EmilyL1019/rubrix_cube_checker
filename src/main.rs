use std::env;
use rubrix_cube_checker::io::{load_cube, parse_moves_file};
use rubrix_cube_checker::movements::apply_move;
use rubrix_cube_checker::compare::moves_to_solved;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // Solve mode
        3 => {
            let cube_file = &args[1];
            let moves_file = &args[2];

            let cube = load_cube(cube_file);
            let mut new_cube = cube.clone();

            // Apply moves from file
            let moves = parse_moves_file(moves_file);
            for m in moves {
                new_cube = apply_move(&new_cube, m.clone());
            }

            println!("Solved cube?: {:?}", new_cube.is_solved());

            match moves_to_solved(&new_cube) {
                Some(solution) => {
                    println!("Solution: {:?}", solution);
                }
                None => {
                    println!("No solution found (cube may be impossible or too deep).");
                }
            }
        }

        // Transform mode
        4 if args[1] == "--apply" => {
            let cube_file = &args[2];
            let moves_file = &args[3];

            let cube = load_cube(cube_file);
            let mut new_cube = cube.clone();

            let moves = parse_moves_file(moves_file);

            for m in moves {
                new_cube = apply_move(&new_cube, m.clone());
            }

            println!("{:?}", new_cube.print_cube());
        }

        _ => {
            eprintln!("Usage:");
            eprintln!("  cargo run <cube_file> <moves_file>");
            eprintln!("      Applies moves, then solves cube");
            eprintln!();
            eprintln!("  cargo run --apply <cube_file> <moves_file>");
            eprintln!("      Applies moves and prints resulting cube");
            std::process::exit(1);
        }
    }
}