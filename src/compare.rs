use crate::cube::{RubriksCube, Face};

pub fn compare(cube1: &RubriksCube, cube2: &RubriksCube) -> i32 {
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