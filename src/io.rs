use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs;
use crate::cube::{RubriksCube, Face, CubeFace};

// List of possible moves
#[derive(Debug)]
pub enum Move {
    U, U1, U2,
    D, D1, D2,
    F, F1, F2,
    B, B1, B2,
    L, L1, L2,
    R, R1, R2
}

// Read file into Vec<Vec<i8>>
fn read_lines(path: &str) -> Vec<Vec<i8>> {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i8>().expect("Invalid number"))
                .collect::<Vec<i8>>()
        })
        .collect()
}

// Build faces from cube
fn build_faces(cube: Vec<Vec<i8>>) -> Vec<CubeFace> {
    let mut faces = Vec::new();

    // Up
    let up = vec![
        cube[0][0], cube[0][1],
        cube[1][0], cube[1][1],
    ];

    // Left
    let left= vec![
        cube[2][0], cube[2][1],
        cube[3][0], cube[3][1],
    ];

    // Front
    let front = vec![
        cube[2][2], cube[2][3],
        cube[3][2], cube[3][3],
    ];

    // Right
    let right = vec![
        cube[2][4], cube[2][5],
        cube[3][4], cube[3][5],
    ];

    // Back
    let back = vec![
        cube[2][6], cube[2][7],
        cube[3][6], cube[3][7],
    ];

    // Down 
    let down = vec![
        cube[4][0], cube[4][1],
        cube[5][0], cube[5][1],
    ];

    faces.push(CubeFace { face_data: up,    orientation: Face::Up });
    faces.push(CubeFace { face_data: left,  orientation: Face::Left });
    faces.push(CubeFace { face_data: front, orientation: Face::Front });
    faces.push(CubeFace { face_data: right, orientation: Face::Right });
    faces.push(CubeFace { face_data: back,  orientation: Face::Back });
    faces.push(CubeFace { face_data: down,  orientation: Face::Down });

    faces
}

// Load cube
pub fn load_cube(path: &str) -> RubriksCube {
    let cube = read_lines(path);
    let faces = build_faces(cube);

    return RubriksCube::new(faces);
}

// Read moves file 
pub fn parse_moves_file(file:&String) -> Vec<Move> {
    let content = fs::read_to_string(file)
        .expect("Failed to read moves file");

    content
        .split_whitespace() // splits on spaces, tabs, AND newlines
        .map(|s| {
            let s = s.to_uppercase();
            match s.as_str() {
                "U"  => Move::U,
                "U1" => Move::U1,
                "U2" => Move::U2,
                "D"  => Move::D,
                "D1" => Move::D1,
                "D2" => Move::D2,
                "F"  => Move::F,
                "F1" => Move::F1,
                "F2" => Move::F2,
                "B"  => Move::B,
                "B1" => Move::B1,
                "B2" => Move::B2,
                "L"  => Move::L,
                "L1" => Move::L1,
                "L2" => Move::L2,
                "R"  => Move::R,
                "R1" => Move::R1,
                "R2" => Move::R2,
                _ => panic!("Invalid move: {}", s),
            }
        })
        .collect()
}