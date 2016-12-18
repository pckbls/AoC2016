extern crate crypto;

use std::str;
use std::env;
use crypto::md5;
use crypto::digest::Digest;

enum Part { One, Two }

fn main() {
    // Parse command line arguments
    let usage = "Usage: <door id> <one|two>";
    let door_id = env::args().nth(1).expect(usage);
    let part = match &env::args().nth(2).expect(usage) as &str {
        "one" => Part::One,
        "two" => Part::Two,
        _ => panic!(usage)
    };

    // Now begin searching for the code sequence.
    let mut door_code = String::from("________").into_bytes();
    let mut found_keys: u8 = 0;
    let mut index: u64 = 0;
    loop {
        // Generate our string to be hashed
        let input_string = door_id.to_string() + &index.to_string();

        // Calculate md5 and obtain results
        let mut m = md5::Md5::new();
        m.input_str(&input_string);
        let hash_str = m.result_str();

        print!("\r{0} Testing: {1} {2}",
            String::from_utf8(door_code.clone()).unwrap(),
            input_string, hash_str);

        if &hash_str[0..5] == "00000" {
            let (index, value) = match part {
                Part::One => (
                    found_keys as usize,
                    hash_str[5..6].as_bytes()[0]
                ),
                Part::Two => (
                    // TODO: unwrap_or statement works but is dirty
                    hash_str[5..6].parse::<usize>().unwrap_or(16),
                    hash_str[6..7].as_bytes()[0]
                )
            };

            if index >= 8 {
                print!("\r\x1B[KInvalid index: {0}\n", index);
            }
            else if door_code[index] != '_' as u8 {
                print!("\r\x1B[KAlready found key for index: {0}\n", index);
            }
            else {
                print!("\r\x1B[KFound key for index: {0}\n", index);
                door_code[index] = value;
                found_keys += 1;
            }

            if found_keys == 8 {
                break;
            }
        }

        index += 1;
    }

    // We're done.
    println!("Door code: {0}", String::from_utf8(door_code).unwrap());
}
