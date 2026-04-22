use std::collections::{HashMap, HashSet, VecDeque};
use crate::cube::{RubrixCube, Face};
use crate::io::Move;
use crate::movements::apply_move;

const MOVES: [Move; 18] = [
    Move::U, Move::U1, Move::U2,
    Move::D, Move::D1, Move::D2,
    Move::F, Move::F1, Move::F2,
    Move::B, Move::B1, Move::B2,
    Move::L, Move::L1, Move::L2,
    Move::R, Move::R1, Move::R2,
];


pub fn is_solved(cube: &RubrixCube) -> bool {
    let faces = [
        Face::Up,
        Face::Down,
        Face::Front,
        Face::Back,
        Face::Left,
        Face::Right,
    ];

    for face in faces {
        let f = match cube.get_face(face) {
            Some(f) => f,
            None => return false,
        };

        let first = f.face_data[0];

        if f.face_data.iter().any(|&x| x != first) {
            return false;
        }
    }

    true
}

pub fn compare(cube1: &RubrixCube, cube2: &RubrixCube) -> i32 {
    let mut diff = 0;

    let faces = [
        Face::Up,
        Face::Down,
        Face::Left,
        Face::Right,
        Face::Front,
        Face::Back,
    ];

    for face in faces {
        let f1 = cube1.get_face(face.clone()).expect("Missing face in cube1");
        let f2 = cube2.get_face(face.clone()).expect("Missing face in cube2");

        for (a, b) in f1.face_data.iter().zip(f2.face_data.iter()) {
            if a != b {
                diff += 1;
            }
        }
    }
    diff
}

fn adj_matrix(cube: &RubrixCube) -> Vec<Vec<u8>> {
    let mut matrix:Vec<Vec<u8>> = vec![];
    // Get cube data
    let [up, down, front, back, left, right] = cube.faces();
    // front adj
    matrix.push(vec![front.face_data[0], up.face_data[2], left.face_data[1]]);
    matrix.push(vec![front.face_data[1], up.face_data[3], right.face_data[0]]);
    matrix.push(vec![front.face_data[2], down.face_data[0], left.face_data[3]]); 
    matrix.push(vec![front.face_data[3], down.face_data[1], right.face_data[2]]);

    // back adj
    matrix.push(vec![back.face_data[0], up.face_data[1], right.face_data[1]]);
    matrix.push(vec![back.face_data[1], up.face_data[0], left.face_data[0]]);
    matrix.push(vec![back.face_data[2], down.face_data[3], right.face_data[3]]); 
    matrix.push(vec![front.face_data[3], down.face_data[2], left.face_data[2]]);
    
    return matrix;
}

fn impossible(cube: &RubrixCube) -> bool {
    //exactly 4 of each color
    let mut counts: HashMap<u8, usize> = HashMap::new();

    for face in &cube.faces {
        for &color in &face.face_data {
            *counts.entry(color).or_insert(0) += 1;
        }
    }

    // 6 colors, each appears 4 times
    if counts.len() != 6 {
        return true;
    }

    if counts.values().any(|&count| count != 4) {
        return true;
    }

     // ----- every corner must contain 3 unique colors -----
    let matrix = adj_matrix(cube);

    for triple in matrix {
        let set: HashSet<u8> = triple.into_iter().collect();

        if set.len() != 3 {
            return true;
        }
    }

    false

}

pub fn moves_to_solved(start: &RubrixCube) -> Option<Vec<Move>> {
    let moves: Vec<Move> = vec![];
    // Call recurrsive helper
    println!("Recursive");
    return moves_solved(start, moves);
}

fn moves_solved(start: &RubrixCube, moves: Vec<Move>) -> Option<Vec<Move>> {
    // If we start with a solved cube, return no moves
    if start.is_solved() {
        return Some(moves);
    }
    // If cube is impossible
    else if impossible(start) {
        return None;
    }

    let mut queue: VecDeque<(RubrixCube, Vec<Move>)> = VecDeque::new();
    queue.push_back((start.clone(), vec![]));

    while let Some((cube, moves)) = queue.pop_front() {
        for mv in MOVES {
            let mut new_moves = moves.clone();
            new_moves.push(mv.clone());

            let next_cube = apply_move(&cube, mv);

            if next_cube.is_solved() {
                return Some(new_moves);
            }

            if !impossible(&next_cube) {
                queue.push_back((next_cube, new_moves));
            }
        }
    }

    None
}