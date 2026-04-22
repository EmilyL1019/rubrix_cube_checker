#[cfg(test)]
mod tests {
    use rubrix_cube_checker::cube::RubrixCube;
    use rubrix_cube_checker::io::Move;
    use rubrix_cube_checker::movements::apply_move;
    use rubrix_cube_checker::compare::{moves_to_solved, compare};

    #[test]
    fn test_is_solved_true() {
        let cube = RubrixCube::new_solved();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_is_solved_false() {
        let mut cube = RubrixCube::new_solved();
        cube = apply_move(&cube, Move::B);
        cube = apply_move(&cube, Move::R);
        assert_eq!(cube.is_solved(), false);
    }

    #[test]
    fn test_compare_solved_vs_scrambled() {
        let solved = RubrixCube::new_solved();

        let mut scrambled = solved.clone();
        scrambled = apply_move(&scrambled, Move::U); // one move scramble

        assert!(compare(&solved, &scrambled) > 0);
    }

    #[test]
    fn test_moves_to_solved_solved_cube() {
        let cube = RubrixCube::new_solved();
        assert_eq!(moves_to_solved(&cube).unwrap().len(), 0);
    }

    #[test]
    fn test_moves_to_solved_one_move_scramble() {
        let solved = RubrixCube::new_solved();

        let mut scrambled = solved.clone();
        scrambled = apply_move(&scrambled, Move::F); // simple scramble

        println!("start");
        let result = moves_to_solved(&scrambled);

        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_moves_to_solved_multi_move_scramble() {
        let solved = RubrixCube::new_solved();

        let mut scrambled = solved.clone();
        scrambled = apply_move(&scrambled, Move::F);
        scrambled = apply_move(&scrambled, Move::B2);
        scrambled = apply_move(&scrambled, Move::L1);
        scrambled = apply_move(&scrambled, Move::U2);
        scrambled = apply_move(&scrambled, Move::D); 

        scrambled.print_cube();

        let result = moves_to_solved(&scrambled);

        assert!(result.is_some());
        let expected = vec![Move::U, Move::F, Move::L];
        assert_eq!(result.unwrap(), expected);
    }
}