use crate::cube::{CubeFace, Face, RubriksCube};
use crate::io::Move;

// Get all face data
fn get_all_face_data(faces: &Vec<CubeFace>) -> (Vec<i8>, Vec<i8>, Vec<i8>, Vec<i8>, Vec<i8>, Vec<i8>){
    let mut up_data = Vec::new();
    let mut front_data = Vec::new();
    let mut left_data = Vec::new();
    let mut back_data = Vec::new();
    let mut right_data = Vec::new();
    let mut down_data = Vec::new();
    
    for face in faces {
        match face.orientation {
            Face::Up => up_data = face.face_data.clone(),
            Face::Left => left_data = face.face_data.clone(),
            Face::Front => front_data = face.face_data.clone(),
            Face::Down => down_data = face.face_data.clone(),
            Face::Right => right_data = face.face_data.clone(),
            Face::Back => back_data = face.face_data.clone(),
        }
    }
    return (up_data, front_data, left_data, back_data, right_data, down_data);
}

// Rotate top face 90 degrees clockwise
fn u(cube: RubriksCube) -> RubriksCube {
    // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate UP face clockwise
    let new_up_data = vec![
        up_data[2], up_data[0],
        up_data[3], up_data[1],
    ];

    let new_front_data = vec![
        right_data[0], right_data[1], 
        front_data[2], front_data[3],
    ];

    let new_right_data = vec![
        back_data[0], back_data[1],
        right_data[2], right_data[3]
    ];

    let new_back_data = vec![
        left_data[0], left_data[1],
        back_data[2], back_data[3]
    ];

    let new_left_data = vec![
        front_data[0], front_data[1],
        left_data[2], left_data[3]
    ];

    let new_faces: Vec<CubeFace> = vec![
    CubeFace {face_data: new_up_data, orientation: Face::Up},
    CubeFace {face_data: new_front_data, orientation: Face::Front},
    CubeFace {face_data: new_left_data, orientation: Face::Left},
    CubeFace {face_data: new_back_data, orientation: Face::Back},
    CubeFace {face_data: new_right_data, orientation: Face::Right},
    CubeFace {face_data: down_data, orientation: Face::Down},
];
    return RubriksCube {faces: new_faces};
}


// Rotate top face 90 degrees counter-clockwise
fn u1(cube: RubriksCube) -> RubriksCube {
   // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate UP face counter-clockwise
    let new_up_data = vec![
        up_data[1], up_data[3],
        up_data[0], up_data[2],
    ];

    let new_front_data = vec![
        left_data[0], left_data[1], 
        front_data[2], front_data[3],
    ];

    let new_right_data = vec![
        front_data[0], front_data[1],
        right_data[2], right_data[3]
    ];

    let new_back_data = vec![
        right_data[0], right_data[1],
        back_data[2], back_data[3]
    ];

    let new_left_data = vec![
        back_data[0], back_data[1],
        left_data[2], left_data[3]
    ];

    let new_faces: Vec<CubeFace> = vec![
    CubeFace {face_data: new_up_data, orientation: Face::Up},
    CubeFace {face_data: new_front_data, orientation: Face::Front},
    CubeFace {face_data: new_left_data, orientation: Face::Left},
    CubeFace {face_data: new_back_data, orientation: Face::Back},
    CubeFace {face_data: new_right_data, orientation: Face::Right},
    CubeFace {face_data: down_data, orientation: Face::Down},
];
    return RubriksCube {faces: new_faces};
}

// Rotate top face 180 degrees 
fn u2(cube: RubriksCube) -> RubriksCube {
    let new_cube_1 = u(cube);
    return u(new_cube_1);
}

// Rotate bottom face 90 degrees clockwise
fn d(cube: RubriksCube) -> RubriksCube {
    // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate DOWN face clockwise
    let new_down_data = vec![
        down_data[2], down_data[0],
        down_data[3], down_data[1],
    ];

    let new_front_data = vec![
        front_data[0], front_data[1], 
        left_data[2], left_data[3],
    ];

    let new_right_data = vec![
        right_data[0], right_data[1],
        front_data[2], front_data[3]
    ];

    let new_back_data = vec![
        back_data[0], back_data[1],
        right_data[2], right_data[3]
    ];

    let new_left_data = vec![
        left_data[0], left_data[1],
        back_data[2], back_data[3]
    ];

    let new_faces: Vec<CubeFace> = vec![
    CubeFace {face_data: up_data, orientation: Face::Up},
    CubeFace {face_data: new_front_data, orientation: Face::Front},
    CubeFace {face_data: new_left_data, orientation: Face::Left},
    CubeFace {face_data: new_back_data, orientation: Face::Back},
    CubeFace {face_data: new_right_data, orientation: Face::Right},
    CubeFace {face_data: new_down_data, orientation: Face::Down},
];
    return RubriksCube {faces: new_faces};
}

// Rotate bottom face 90 degrees counter-clockwise
fn d1(cube: RubriksCube) -> RubriksCube {
   // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate DOWN face clockwise
    let new_down_data = vec![
        down_data[1], down_data[3],
        down_data[0], down_data[2],
    ];

    let new_front_data = vec![
        front_data[0], front_data[1], 
        right_data[2], right_data[3],
    ];

    let new_right_data = vec![
        right_data[0], right_data[1],
        back_data[2], back_data[3]
    ];

    let new_back_data = vec![
        back_data[0], back_data[1],
        left_data[2], left_data[3]
    ];

    let new_left_data = vec![
        left_data[0], left_data[1],
        front_data[2], front_data[3]
    ];

    let new_faces: Vec<CubeFace> = vec![
        CubeFace {face_data: up_data, orientation: Face::Up},
        CubeFace {face_data: new_front_data, orientation: Face::Front},
        CubeFace {face_data: new_left_data, orientation: Face::Left},
        CubeFace {face_data: new_back_data, orientation: Face::Back},
        CubeFace {face_data: new_right_data, orientation: Face::Right},
        CubeFace {face_data: new_down_data, orientation: Face::Down},
    ];
    return RubriksCube {faces: new_faces};
}

// Rotate DOWN face 180 degrees 
fn d2(cube: RubriksCube) -> RubriksCube {
    // This is two d moves
    return d(d(cube));
}

// Rotate front face 90 degrees clockwise
fn f(cube: RubriksCube) -> RubriksCube {
    // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate FRONT face clockwise
    let new_front_data = vec![
        front_data[2], front_data[0],
        front_data[3], front_data[1],
    ];

    let new_up_data = vec![
        up_data[0], up_data[1], 
        left_data[3], left_data[1],
    ];

    let new_right_data = vec![
        up_data[2], right_data[1],
        up_data[3], right_data[3]
    ];

    let new_down_data = vec![
        right_data[2], right_data[0],
        down_data[2], down_data[3]
    ];

    let new_left_data = vec![
        left_data[0], down_data[0],
        left_data[2], down_data[1]
    ];

    let new_faces: Vec<CubeFace> = vec![
        CubeFace {face_data: new_up_data, orientation: Face::Up},
        CubeFace {face_data: new_front_data, orientation: Face::Front},
        CubeFace {face_data: new_left_data, orientation: Face::Left},
        CubeFace {face_data: back_data, orientation: Face::Back},
        CubeFace {face_data: new_right_data, orientation: Face::Right},
        CubeFace {face_data: new_down_data, orientation: Face::Down},
    ];

    return RubriksCube {faces: new_faces};
}

// Rotate front face 90 degrees counter-clockwise
fn f1(cube: RubriksCube) -> RubriksCube {
    // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate FRONT face counter-clockwise
    let new_front_data = vec![
        front_data[1], front_data[3],
        front_data[0], front_data[2],
    ];

    let new_up_data = vec![
        up_data[0], up_data[1], 
        right_data[0], right_data[2],
    ];

    let new_right_data = vec![
        down_data[1], right_data[1],
        down_data[0], right_data[3]
    ];

    let new_down_data = vec![
        left_data[1], left_data[3],
        down_data[2], down_data[3]
    ];

    let new_left_data = vec![
        left_data[0], up_data[3],
        left_data[2], up_data[2]
    ];

    let new_faces: Vec<CubeFace> = vec![
        CubeFace {face_data: new_up_data, orientation: Face::Up},
        CubeFace {face_data: new_front_data, orientation: Face::Front},
        CubeFace {face_data: new_left_data, orientation: Face::Left},
        CubeFace {face_data: back_data, orientation: Face::Back},
        CubeFace {face_data: new_right_data, orientation: Face::Right},
        CubeFace {face_data: new_down_data, orientation: Face::Down},
    ];
    return RubriksCube {faces: new_faces};
}

// Rotate front face 180 degrees
fn f2(cube: RubriksCube) -> RubriksCube {
    // This is 2 f moves
    return f(f(cube));
}

// Rotate BACK face 90 degrees clockwise
fn b(cube: RubriksCube) -> RubriksCube {
    // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate BACK face clockwise
    let new_back_data = vec![
        back_data[2], back_data[0],
        back_data[3], back_data[1],
    ];

    let new_up_data = vec![
        right_data[1], right_data[3], 
        up_data[2], up_data[3],
    ];

    let new_right_data = vec![
        right_data[0], down_data[3],
        right_data[2], down_data[2]
    ];

    let new_down_data = vec![
        down_data[0], down_data[1],
        left_data[0], left_data[2]
    ];

    let new_left_data = vec![
        up_data[1], left_data[1], 
        up_data[0], left_data[3],
    ];

    let new_faces: Vec<CubeFace> = vec![
        CubeFace {face_data: new_up_data, orientation: Face::Up},
        CubeFace {face_data: front_data, orientation: Face::Front},
        CubeFace {face_data: new_left_data, orientation: Face::Left},
        CubeFace {face_data: new_back_data, orientation: Face::Back},
        CubeFace {face_data: new_right_data, orientation: Face::Right},
        CubeFace {face_data: new_down_data, orientation: Face::Down},
    ];
    return RubriksCube {faces: new_faces};
}

// Rotate BACK face 90 degrees counter clockwise
fn b1(cube: RubriksCube) -> RubriksCube {
    // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate BACK face clockwise
    let new_back_data = vec![
        back_data[1], back_data[3],
        back_data[0], back_data[2],
    ];

    let new_up_data = vec![
        left_data[2], left_data[0], 
        up_data[2], up_data[3],
    ];

    let new_right_data = vec![
        right_data[0], up_data[0],
        right_data[2], up_data[1]
    ];

    let new_down_data = vec![
        down_data[0], down_data[1],
        right_data[3], right_data[1]
    ];

    let new_left_data = vec![
        down_data[2], left_data[1], 
        down_data[3], left_data[3],
    ];

    let new_faces: Vec<CubeFace> = vec![
        CubeFace {face_data: new_up_data, orientation: Face::Up},
        CubeFace {face_data: front_data, orientation: Face::Front},
        CubeFace {face_data: new_left_data, orientation: Face::Left},
        CubeFace {face_data: new_back_data, orientation: Face::Back},
        CubeFace {face_data: new_right_data, orientation: Face::Right},
        CubeFace {face_data: new_down_data, orientation: Face::Down},
    ];
    return RubriksCube {faces: new_faces};
}

// Rotate BACK face 180 degrees
fn b2(cube: RubriksCube) -> RubriksCube {
    // This is 2 b moves
    return b(b(cube));
}

// Rotate left face 90 degrees clockwise
fn l(cube: RubriksCube) -> RubriksCube {
    // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);
    
    // Rotate LEFT face clockwise
    let new_left_data = vec![
        left_data[2], left_data[0],
        left_data[3], left_data[1],
    ];

    let new_up_data = vec![
        back_data[3], up_data[1], 
        back_data[1], up_data[3],
    ];

    let new_front_data = vec![
        up_data[0], front_data[1],
        up_data[2], front_data[3]
    ];

    let new_down_data = vec![
        front_data[0], down_data[1],
        front_data[2], down_data[3]
    ];

    let new_back_data = vec![
        back_data[0], down_data[2],
        back_data[2], down_data[0]
    ];

    let new_faces: Vec<CubeFace> = vec![
        CubeFace {face_data: new_up_data, orientation: Face::Up},
        CubeFace {face_data: new_front_data, orientation: Face::Front},
        CubeFace {face_data: new_left_data, orientation: Face::Left},
        CubeFace {face_data: new_back_data, orientation: Face::Back},
        CubeFace {face_data: right_data, orientation: Face::Right},
        CubeFace {face_data: new_down_data, orientation: Face::Down},
    ];
    return RubriksCube {faces: new_faces};
}

// Rotate left face 90 degrees counter clockwise
fn l1(cube: RubriksCube) -> RubriksCube {
    // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate LEFT face counterclockwise
    let new_left_data = vec![
        left_data[1], left_data[3],
        left_data[0], left_data[2],
    ];

    let new_up_data = vec![
        front_data[0], up_data[1], 
        front_data[2], up_data[3],
    ];

    let new_front_data = vec![
        down_data[0], front_data[1],
        down_data[2], front_data[3]
    ];

    let new_down_data = vec![
        back_data[3], down_data[1],
        back_data[1], down_data[3]
    ];

    let new_back_data = vec![
        back_data[0], up_data[2],
        back_data[2], up_data[0]
    ];

    let new_faces: Vec<CubeFace> = vec![
        CubeFace {face_data: new_up_data, orientation: Face::Up},
        CubeFace {face_data: new_front_data, orientation: Face::Front},
        CubeFace {face_data: new_left_data, orientation: Face::Left},
        CubeFace {face_data: new_back_data, orientation: Face::Back},
        CubeFace {face_data: right_data, orientation: Face::Right},
        CubeFace {face_data: new_down_data, orientation: Face::Down},
    ];
    return RubriksCube {faces: new_faces};
}

// Rotate left face 180 degrees 
fn l2(cube: RubriksCube) -> RubriksCube {
    // This is 2 l moves
    return l(l(cube));
}

// Rotate right face 90 degrees clockwise
fn r(cube: RubriksCube) -> RubriksCube {
    // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate RIGHT face clockwise
    let new_right_data = vec![
        right_data[2], right_data[0],
        right_data[3], right_data[1],
    ];

    let new_up_data = vec![
        up_data[0], front_data[1], 
        up_data[2], front_data[3],
    ];

    let new_front_data = vec![
        front_data[0], down_data[1],
        front_data[2], down_data[3]
    ];

    let new_down_data = vec![
        down_data[0], back_data[2],
        down_data[2], back_data[0]
    ];

    let new_back_data = vec![
        up_data[3], back_data[1],
        up_data[1], back_data[3]
    ];

    let new_faces: Vec<CubeFace> = vec![
        CubeFace {face_data: new_up_data, orientation: Face::Up},
        CubeFace {face_data: new_front_data, orientation: Face::Front},
        CubeFace {face_data: left_data, orientation: Face::Left},
        CubeFace {face_data: new_back_data, orientation: Face::Back},
        CubeFace {face_data: new_right_data, orientation: Face::Right},
        CubeFace {face_data: new_down_data, orientation: Face::Down},
    ];
    get_all_face_data(&new_faces);
    return RubriksCube {faces: new_faces};
}


// Rotate right face 90 degrees clockwise
fn r1(cube: RubriksCube) -> RubriksCube {
    // Extract face data 
    let (up_data, front_data, left_data, back_data, right_data, down_data) = get_all_face_data(&cube.faces);

    // Rotate RIGHT face counter-clockwise
    let new_right_data = vec![
        right_data[1], right_data[3],
        right_data[0], right_data[2],
    ];

    let new_up_data = vec![
        up_data[0], back_data[2], 
        up_data[2], back_data[0],
    ];

    let new_front_data = vec![
        front_data[0], up_data[1],
        front_data[2], up_data[3]
    ];

    let new_down_data = vec![
        down_data[0], front_data[1],
        down_data[2], front_data[3]
    ];

    let new_back_data = vec![
        down_data[3], back_data[1],
        down_data[1], back_data[3]
    ];

    let new_faces: Vec<CubeFace> = vec![
        CubeFace {face_data: new_up_data, orientation: Face::Up},
        CubeFace {face_data: new_front_data, orientation: Face::Front},
        CubeFace {face_data: left_data, orientation: Face::Left},
        CubeFace {face_data: new_back_data, orientation: Face::Back},
        CubeFace {face_data: new_right_data, orientation: Face::Right},
        CubeFace {face_data: new_down_data, orientation: Face::Down},
    ];
    get_all_face_data(&new_faces);
    return RubriksCube {faces: new_faces};
}

// Rotate right face 180 degrees 
fn r2(cube: RubriksCube) -> RubriksCube {
    // This is 2 r moves
    return r(r(cube));
}

// Perform one move on the Rubriks cube
pub fn apply_move(cube: RubriksCube, cube_move: Move) -> RubriksCube {
    return match cube_move {
        Move::U  => u(cube),
        Move::U1 => u1(cube),
        Move::U2 => u2(cube),

        Move::F  => f(cube),
        Move::F1 => f1(cube),
        Move::F2 => f2(cube),

        Move::R  => r(cube),
        Move::R1 => r1(cube),
        Move::R2 => r2(cube),

        Move::D => d(cube),
        Move::D1 => d1(cube),
        Move::D2 => d2(cube),

        Move::B => b(cube),
        Move::B1 => b1(cube),
        Move::B2 => b2(cube),

        Move::L => l(cube),
        Move::L1 => l1(cube),
        Move::L2 => l2(cube),
    };
}