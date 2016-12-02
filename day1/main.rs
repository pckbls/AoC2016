use std::io;
use std::convert;

#[derive(Copy, Clone)]
enum Orientation { North = 0, East = 1, South = 2, West = 3 }
impl convert::From<i32> for Orientation {
    fn from(x: i32) -> Self {
        match x {
            0 => Orientation::North,
            1 => Orientation::East,
            2 => Orientation::South,
            3 => Orientation::West,
            _ => panic!("Foo")
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position { x: i32, y: i32 }

enum Rotation { Left, Right }

struct Me { orientation: Orientation, position: Position }
impl Me {
    fn rotate(&mut self, r: &Rotation) {
        // First convert the orientation into an integer.
        let mut i = self.orientation as i32;

        // Now apply the rotation.
        i = match *r {
            Rotation::Left => (i - 1) % 4,
            Rotation::Right => (i + 1) % 4,
        };

        // Rust's modulo implementation for negative numbers results in another negative number
        // However we want i to stay in [0, 3].
        if i < 0 {
            i += 4;
        }

        // Finally convert the integer back into our Orientation enum.
        self.orientation = Orientation::from(i);
    }

    fn walk(&mut self) {
        match self.orientation {
            Orientation::North => self.position.y += 1,
            Orientation::East => self.position.x += 1,
            Orientation::South => self.position.y -= 1,
            Orientation::West => self.position.x -= 1,
        }
    }
}

struct Sequence { rotation: Rotation, steps: u32 }

fn main() {
    // Read data from stdin.
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Cannot read from stdin.");

    // Parse standard input and populate our sequences vector.
    let mut sequences: Vec<Sequence> = Vec::new();
    for mut sequence_string in input.split(",") {
        sequence_string = sequence_string.trim();

        let sequence = Sequence {
            rotation: match &sequence_string[0..1] {
                "L" => Rotation::Left,
                "R" => Rotation::Right,
                _ => panic!("Expected turn direction.")
            },

            steps: match sequence_string[1..].parse() {
                Ok(num) => num,
                Err(_) => panic!("Expected number of steps.")
            }
        };

        sequences.push(sequence);
    }

    // Initialize ourself.
    let mut me = Me {
        orientation: Orientation::North,
        position: Position { x: 0, y: 0 }
    };

    // Initialize position history.
    let mut history: Vec<Position> = Vec::new();
    history.push(me.position);

    // Move around
    'outer: for sequence in &sequences {
        me.rotate(&sequence.rotation);

        for _ in 0..sequence.steps {
            me.walk();

            // Check if we've already been at this position before.
            for &past_position in &history {
                if past_position == me.position {
                    println!("We've been here before!");
                    break 'outer;
                }
            }

            history.push(me.position);
        }
    }

    // Determine distance from origin.
    let distance = me.position.x.abs() + me.position.y.abs();

    println!("Current position: {:?}", me.position);
    println!("Distance from origin: {}", distance);
}

