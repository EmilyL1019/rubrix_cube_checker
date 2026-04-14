// A representation of Rubriks cube
#[derive(PartialEq, Clone)]
pub enum Face {
    Front,
    Back,
    Up,
    Down, 
    Left,
    Right
}

// This represents one face of a cube. It has the colors of the 4 cells in row-major order.
#[derive(PartialEq)]
pub struct CubeFace {
    pub face_data: Vec<i8>,
    pub orientation: Face
}

// This represents the entire Rubriks cube.
pub struct RubriksCube {
    pub faces: Vec<CubeFace>
}

impl RubriksCube {
    // Builds a new cube out of the cube's data
    pub fn new(faces: Vec<CubeFace>) -> RubriksCube {   
        let cube: RubriksCube = RubriksCube {faces};
        return cube;
    }

    // Get data from 1 given face
    pub fn get_face(&self, target: Face) -> Option<&CubeFace> {
        for face in &self.faces {
            if face.orientation == target {
                return Some(face);
            }
        }
        None
    }

    // Print Rubix cube for testing
    pub fn print_cube(&self) {
        let up = self.get_face(Face::Up).unwrap();
        let down = self.get_face(Face::Down).unwrap();
        let front = self.get_face(Face::Front).unwrap();
        let back = self.get_face(Face::Back).unwrap();
        let left = self.get_face(Face::Left).unwrap();
        let right = self.get_face(Face::Right).unwrap();

        // Helper closure to access values
        let f = |face: &CubeFace, i: usize| face.face_data[i];

        // Top
        println!("      {} {}", f(up, 0), f(up, 1));
        println!("      {} {}", f(up, 2), f(up, 3));

        // Middle (Left, Front, Right, Back)
        println!(
            "{} {}  {} {}  {} {}  {} {}",
            f(left, 0), f(left, 1),
            f(front, 0), f(front, 1),
            f(right, 0), f(right, 1),
            f(back, 0), f(back, 1),
        );
        println!(
            "{} {}  {} {}  {} {}  {} {}",
            f(left, 2), f(left, 3),
            f(front, 2), f(front, 3),
            f(right, 2), f(right, 3),
            f(back, 2), f(back, 3),
        );

        // Bottom
        println!("      {} {}", f(down, 0), f(down, 1));
        println!("      {} {}", f(down, 2), f(down, 3));
    }
}
