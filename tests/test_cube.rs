use rubrix_cube_checker::io::{load_cube, parse_moves_file};
use rubrix_cube_checker::movements::apply_move;
use rubrix_cube_checker::compare::compare;

// ----------------------------
// Shared test runner
// ----------------------------
fn run_test(move_type: &str, move_name: &str, cube_prefix: &str) {
    let base = env!("CARGO_MANIFEST_DIR");

    let cube_file = format!("{}/tests/{}.txt", base, cube_prefix);
    println!("cube_file = {}", cube_file);

    let mut cube = load_cube(&cube_file);

    let move_file = format!("{}/tests/moves/{}_moves/{}.txt", base, move_type, move_name);

    let expected_file = format!(
        "{}/tests/{}_transformations/{}_moves/{}_{}_correct.txt",
        base, cube_prefix, move_type, cube_prefix, move_name
    );
    println!("{}, {}", move_file, expected_file);

    let expected = load_cube(&expected_file);
    let moves = parse_moves_file(&move_file, true);

    for m in moves {
        cube = apply_move(&cube, m);
    }

    let diff = compare(&cube, &expected);

    assert_eq!(diff, 0);
}

// ----------------------------
// Macro (FIXED: consistent args)
// ----------------------------
macro_rules! move_test {
    ($name:ident, $cube:expr, $move:expr, $single:expr) => {
        #[test]
        fn $name() {
            run_test($single, $move, $cube);
        }
    };
}

// =======================================================
// CUBE 1 TESTS
// =======================================================

move_test!(test_c1_f,  "rubrix1", "f",  "single");
move_test!(test_c1_f1, "rubrix1", "f1", "single");
move_test!(test_c1_f2, "rubrix1", "f2", "single");
move_test!(test_c1_ff1,"rubrix1", "ff1","multi");

move_test!(test_c1_r,  "rubrix1", "r",  "single");
move_test!(test_c1_r1, "rubrix1", "r1", "single");
move_test!(test_c1_r2, "rubrix1", "r2", "single");
move_test!(test_c1_r1r,"rubrix1", "r1r","multi");

move_test!(test_c1_l,  "rubrix1", "l",  "single");
move_test!(test_c1_l1, "rubrix1", "l1", "single");
move_test!(test_c1_l2, "rubrix1", "l2", "single");
move_test!(test_c1_l1l,"rubrix1", "l1l","multi");

move_test!(test_c1_u,  "rubrix1", "u",  "single");
move_test!(test_c1_u1, "rubrix1", "u1", "single");
move_test!(test_c1_u2, "rubrix1", "u2", "single");
move_test!(test_c1_u1u,"rubrix1", "u1u", "multi");

move_test!(test_c1_d,  "rubrix1", "d",  "single");
move_test!(test_c1_d1, "rubrix1", "d1", "single");
move_test!(test_c1_d2, "rubrix1", "d2", "single");
move_test!(test_c1_dd1,"rubrix1", "dd1","multi");

move_test!(test_c1_b,  "rubrix1", "b",  "single");
move_test!(test_c1_b1, "rubrix1", "b1", "single");
move_test!(test_c1_b2, "rubrix1", "b2", "single");
move_test!(test_c1_b1b,"rubrix1", "b1b","multi");

move_test!(test_c1_dr2f1, "rubrix1", "dr2f1", "multi");

// =======================================================
// CUBE 2 TESTS
// =======================================================

move_test!(test_c2_f,  "rubrix2", "f",  "single");
move_test!(test_c2_f1, "rubrix2", "f1", "single");
move_test!(test_c2_f2, "rubrix2", "f2", "single");
move_test!(test_c2_ff1,"rubrix2", "ff1","multi");

move_test!(test_c2_r,  "rubrix2", "r",  "single");
move_test!(test_c2_r1, "rubrix2", "r1", "single");
move_test!(test_c2_r2, "rubrix2", "r2", "single");
move_test!(test_c2_r1r,"rubrix2", "r1r","multi");

move_test!(test_c2_l,  "rubrix2", "l",  "single");
move_test!(test_c2_l1, "rubrix2", "l1", "single");
move_test!(test_c2_l2, "rubrix2", "l2", "single");
move_test!(test_c2_l1l,"rubrix2", "l1l","multi");

move_test!(test_c2_u,  "rubrix2", "u",  "single");
move_test!(test_c2_u1, "rubrix2", "u1", "single");
move_test!(test_c2_u2, "rubrix2", "u2", "single");
move_test!(test_c2_u1u,"rubrix2", "u1u","multi");

move_test!(test_c2_d,  "rubrix2", "d",  "single");
move_test!(test_c2_d1, "rubrix2", "d1", "single");
move_test!(test_c2_d2, "rubrix2", "d2", "single");
move_test!(test_c2_dd1,"rubrix2", "dd1","multi");

move_test!(test_c2_b,  "rubrix2", "b",  "single");
move_test!(test_c2_b1, "rubrix2", "b1", "single");
move_test!(test_c2_b2, "rubrix2", "b2", "single");
move_test!(test_c2_b1b,"rubrix2", "b1b","multi");

move_test!(test_c2_dr2f1, "rubrix2", "dr2f1", "multi");