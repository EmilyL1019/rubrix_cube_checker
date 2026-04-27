use std::env;
use std::path::Path;
use rubrix_cube_checker::io::{Move, load_cube};
use rubrix_cube_checker::movements::apply_move;
use rubrix_cube_checker::compare::moves_to_solved;

fn moves_to_string(moves: Vec<Move>) -> String {
    let mut move_str = String::new();

    for mv in moves {
        let s = match mv {
            Move::B => "B",
            Move::B1 => "B1",
            Move::B2 => "B2",
            Move::D => "D",
            Move::D1 => "D1",
            Move::D2 => "D2",
            Move::F => "F",
            Move::F1 => "F1",
            Move::F2 => "F2",
            Move::L => "L",
            Move::L1 => "L1",
            Move::L2 => "L2",
            Move::R => "R",
            Move::R1 => "R1",
            Move::R2 => "R2",
            Move::U => "U",
            Move::U1 => "U1",
            Move::U2 => "U2",
        };

        move_str.push_str(s);
        move_str.push(' ');
    }

    move_str.trim_end().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // Solve mode
        2 => {
            let cube_file = &args[1];

            let cube = load_cube(cube_file);
            let solve = moves_to_solved(&cube);

            match solve {
                Some(mv) => println!(
                    "{},{}",
                    mv.len(), moves_to_string(mv)
                ),
                None => println!("Unsolvable!"),
            }
        }

        // Distance mode (file-based, unchanged if you still use it elsewhere)
        3 => {
            let cube_file = &args[1];
            let moves_file = &args[2];

            let cube = load_cube(cube_file);
            let mut new_cube = cube.clone();

            let moves = rubrix_cube_checker::io::parse_moves_file(moves_file, true);

            for m in &moves {
                new_cube = apply_move(&new_cube, m.clone());
            }

            println!("Solved cube?: {:?}", new_cube.is_solved());
            let best_moves = match moves_to_solved(&cube) {
                Some(m) => m,
                None => {
                    eprintln!("No solution found");
                    vec![]
                }
            };
            let moves_to_sol = match moves_to_solved(&new_cube) {
                Some(m) => m,
                None => {
                    eprintln!("No solution found");
                    vec![]
                }
            };
            if new_cube.is_solved() {
                if best_moves.len() < moves.len() {
                    println!("Minimum required steps: {}. LLM Steps: {}", best_moves.len(), moves.len())
                }
            }
            else {
                println!("Correct moves: {:?}, Moves Needed with LLM Steps: {:?}", best_moves, moves_to_sol);
            }
        }

        // APPLY MODE (UPDATED: now uses STRING, not file)
        4 if args[1] == "--apply" => {
            let cube_file = &args[2];
            let moves_file = &args[3];

            let cube = load_cube(cube_file);
            let mut new_cube = cube.clone();

            let moves = rubrix_cube_checker::io::parse_moves_file(moves_file, Path::new(moves_file).is_file());

            for m in moves {
                new_cube = apply_move(&new_cube, m);
            }

            // IMPORTANT: only output cube (no debug prints!)
            new_cube.print_cube();
        }

        _ => {
            eprintln!("Usage:");
            eprintln!("  cargo run <cube_file>");
            eprintln!("  cargo run <cube_file> <moves_file>");
            eprintln!("  cargo run --apply <cube_file> \"U R F ...\"");
            std::process::exit(1);
        }
    }
}