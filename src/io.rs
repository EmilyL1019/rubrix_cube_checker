use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs;
use crate::cube::{RubrixCube, Face, CubeFace};

// List of possible moves
#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    U, U1, U2,
    D, D1, D2,
    F, F1, F2,
    B, B1, B2,
    L, L1, L2,
    R, R1, R2
}

/// Parse face label into enum
fn parse_face_label(label: &str) -> Face {
    match label.trim() {
        "U" => Face::Up,
        "D" => Face::Down,
        "F" => Face::Front,
        "B" => Face::Back,
        "L" => Face::Left,
        "R" => Face::Right,
        _ => panic!("Invalid face label: {}", label),
    }
}

/// Read cube file into a HashMap<Face, Vec<u8>>
fn read_faces(path: &str) -> HashMap<Face, Vec<u8>> {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let line = reader
        .lines()
        .next()
        .expect("Empty file")
        .expect("Bad line");

    let mut map: HashMap<Face, Vec<u8>> = HashMap::new();

    for part in line.split(',') {
        let part = part.trim();

        let mut split = part.splitn(2, ':');
        
        let label = split
            .next()
            .expect("Missing label")
            .trim();

        let data = split
            .next()
            .expect("Missing face data")
            .trim();

        let face_vals: Vec<u8> = data
            .replace(';', " ")
            .split_whitespace()
            .map(|s| s.parse::<u8>().expect("Invalid number"))
            .collect();

        if face_vals.len() != 4 {
            panic!("Each face must have exactly 4 values");
        }

        let face = parse_face_label(label);
        map.insert(face, face_vals);
    }

    map
}

/// Build CubeFace vector in fixed internal order
fn build_faces(map: HashMap<Face, Vec<u8>>) -> Vec<CubeFace> {
    vec![
        CubeFace { face_data: map[&Face::Up].clone(),    orientation: Face::Up },
        CubeFace { face_data: map[&Face::Left].clone(),  orientation: Face::Left },
        CubeFace { face_data: map[&Face::Front].clone(), orientation: Face::Front },
        CubeFace { face_data: map[&Face::Right].clone(), orientation: Face::Right },
        CubeFace { face_data: map[&Face::Back].clone(),  orientation: Face::Back },
        CubeFace { face_data: map[&Face::Down].clone(),  orientation: Face::Down },
    ]
}

/// Load full cube
pub fn load_cube(path: &str) -> RubrixCube {
    let map = read_faces(path);
    let faces = build_faces(map);
    RubrixCube::new(faces)
}

// Read moves file 
pub fn parse_moves_file(file:&str, is_file: bool) -> Vec<Move> {
    let content:String = match is_file {
        true => {
            fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("Failed to read moves file {}: {}", file, e))
        }
        false => { file.to_string()}
    };
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