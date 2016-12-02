use std::io;
use std::io::prelude::*;
use std::fmt;

enum Direction { Up, Down, Left, Right }
enum Action { Move(Direction), Press }

#[derive(Copy, Clone)]
struct Keypad { x: i32, y: i32 }
impl Keypad {
    fn move_finger(&mut self, direction: &Direction) {
        let new_keypad = match *direction {
            Direction::Up => Keypad { x: self.x, y: self.y - 1 },
            Direction::Down => Keypad { x: self.x, y: self.y + 1 },
            Direction::Left => Keypad { x: self.x - 1 , y: self.y },
            Direction::Right => Keypad { x: self.x + 1, y: self.y },
        };

        if let Ok(_) = Keypad::position_to_key(&new_keypad) {
            self.x = new_keypad.x;
            self.y = new_keypad.y;
        }
    }

    fn press_finger(&self) {
        print!("{}", self);
    }

    fn position_to_key(keypad: &Keypad) -> Result<u16, &'static str> {
        match *keypad {
            Keypad { x: -1, y: -1 } => Ok(1),
            Keypad { x: 0,  y: -1 } => Ok(2),
            Keypad { x: 1,  y: -1 } => Ok(3),
            Keypad { x: -1, y: 0  } => Ok(4),
            Keypad { x: 0,  y: 0  } => Ok(5),
            Keypad { x: 1,  y: 0  } => Ok(6),
            Keypad { x: -1, y: 1 } => Ok(7),
            Keypad { x: 0,  y: 1 } => Ok(8),
            Keypad { x: 1,  y: 1 } => Ok(9),
            Keypad { .. } => Err("Position outside of key pad boundaries")
        }
    }
}
impl fmt::Display for Keypad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let key = match Keypad::position_to_key(&self) {
            Ok(num) => num,
            Err(_) => panic!("This should not happen.")
        };

        write!(f, "{}", key)
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

    // Initialize key pad.
    let mut keypad = Keypad { x: 0, y: 0 };

    // Perform all the actions.
    for action in &actions {
        match *action {
            Action::Move(ref direction) => keypad.move_finger(direction),
            Action::Press => keypad.press_finger(),
        };
    }

    println!("");
}

