use std::env;
use std::io;
use std::io::prelude::*;

enum Direction { Up, Down, Left, Right }
enum Action { Move(Direction), Press }
struct Position { x: i32, y: i32 }

// Our finger has a position and is attached to a key pad.
// It can move around in any of the four directions and push a key.
struct Finger { position: Position, keypad_func: KeypadFunc }
impl Finger {
    fn move_in_direction(&mut self, direction: &Direction) {
        let new_position = match *direction {
            Direction::Up => Position { x: self.position.x, y: self.position.y - 1 },
            Direction::Down => Position { x: self.position.x, y: self.position.y + 1 },
            Direction::Left => Position { x: self.position.x - 1 , y: self.position.y },
            Direction::Right => Position { x: self.position.x + 1, y: self.position.y },
        };

        if let Ok(_) = (self.keypad_func)(&new_position) {
            self.position = new_position;
        }
    }

    fn push_key(&self) {
        print!("{}", (self.keypad_func)(&self.position).unwrap());
    }
}

// The KeypadFunc maps a Position onto a key.
// If the position is outside the key pad then it shall return an error.
// That way we can clamp the finger position inside the boundaries
// of our key pad.
type KeypadFunc = fn(&Position) -> Result<char, &'static str>;

fn keypad_one(position: &Position) -> Result<char, &'static str> {
    match *position {
        Position { x: -1, y: -1 } => Ok('1'),
        Position { x: 0,  y: -1 } => Ok('2'),
        Position { x: 1,  y: -1 } => Ok('3'),
        Position { x: -1, y: 0  } => Ok('4'),
        Position { x: 0,  y: 0  } => Ok('5'),
        Position { x: 1,  y: 0  } => Ok('6'),
        Position { x: -1, y: 1 } => Ok('7'),
        Position { x: 0,  y: 1 } => Ok('8'),
        Position { x: 1,  y: 1 } => Ok('9'),
        Position { .. } => Err("Position outside of key pad boundaries")
    }
}

fn keypad_two(position: &Position) -> Result<char, &'static str> {
    match *position {
        Position { x: 0, y: -2 } => Ok('1'),
        Position { x: -1, y: -1 } => Ok('2'),
        Position { x: 0, y: -1 } => Ok('3'),
        Position { x: 1, y: -1 } => Ok('4'),
        Position { x: -2, y: 0 } => Ok('5'),
        Position { x: -1, y: 0 } => Ok('6'),
        Position { x: 0, y: 0 } => Ok('7'),
        Position { x: 1, y: 0 } => Ok('8'),
        Position { x: 2, y: 0 } => Ok('9'),
        Position { x: -1, y: 1 } => Ok('A'),
        Position { x: 0, y: 1 } => Ok('B'),
        Position { x: 1, y: 1 } => Ok('C'),
        Position { x: 0, y: 2 } => Ok('D'),
        Position { .. } => Err("Position outside of key pad boundaries")
    }
}

fn main() {
    // Initialize our key pad based on the command line arguments.
    let keypad_func: KeypadFunc = match env::args().nth(1) {
        Some(ref x) if x == "one" => keypad_one,
        Some(ref x) if x == "two" => keypad_two,
        _ => {
            println!("Usage: {} <one|two>", env::args().nth(0).unwrap());
            println!("Two first argument specifies the key pad to use.");
            return;
        }
    };

    // Initialize our actions vector.
    let mut actions: Vec<Action> = Vec::new();

    // Read data from stdin and populate the actions vector.
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for c in line.unwrap().chars() {
            let direction: Direction = match c {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Expected character specifying direction")
            };

            actions.push(Action::Move(direction));
        }

        actions.push(Action::Press);
    }

    // Initialize our finger.
    let mut finger = Finger {
        position: Position { x: 0, y: 0 },
        keypad_func: keypad_func
    };

    // Perform all the actions.
    for action in &actions {
        match *action {
            Action::Move(ref direction) => finger.move_in_direction(direction),
            Action::Press => finger.push_key(),
        };
    }

    println!("");
}

