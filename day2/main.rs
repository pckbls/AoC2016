use std::io;
use std::io::prelude::*;

enum Direction { Up, Down, Left, Right }
enum Action { Move(Direction), Press }
struct Position { x: i32, y: i32 }

// The Keypad trait needs exactly one method to be implemented:
// It shall return a key based on a position with two coordinates.
// If the position is outside the key pad then it shall return an error.
// That way we can determine the boundaries of our key pad when moving
// the finger.
trait Keypad {
    fn position_to_key(&self, &Position) -> Result<char, &'static str>;
}

// The implementation of the key pad for the first part.
struct KeypadOne { }
impl Keypad for KeypadOne {
    fn position_to_key(&self, position: &Position) -> Result<char, &'static str> {
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
}

// Our finger has a position and is attached to a key pad.
// It can move around in any of the four directions and push a key.
struct Finger<K> { position: Position, keypad: K }
impl<K: Keypad> Finger<K> {
    fn move_in_direction(&mut self, direction: &Direction) {
        let new_position = match *direction {
            Direction::Up => Position { x: self.position.x, y: self.position.y - 1 },
            Direction::Down => Position { x: self.position.x, y: self.position.y + 1 },
            Direction::Left => Position { x: self.position.x - 1 , y: self.position.y },
            Direction::Right => Position { x: self.position.x + 1, y: self.position.y },
        };

        if let Ok(_) = self.keypad.position_to_key(&new_position) {
            self.position = new_position;
        }
    }

    fn push_key(&self) {
        print!("{}", self.keypad.position_to_key(&self.position).unwrap());
    }
}

fn main() {
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

    // Initialize our key pad.
    let keypad = KeypadOne { };

    // Initialize our finger.
    let mut finger = Finger {
        position: Position { x: 0, y: 0 },
        keypad: keypad
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

