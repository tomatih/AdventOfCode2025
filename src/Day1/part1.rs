use std::io;

use anyhow::Result;

const DIAL_SIZE: i32 = 100;

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut dial_pos: i32 = 50;
    let mut output = 0;

    for line in stdin.lines() {
        let line = line?;

        let direction = line.chars().nth(0).unwrap();
        let amount = &line[1..].parse::<i32>()?;

        match direction {
            'R' => {
                dial_pos += amount;
            }
            'L' => {
                dial_pos -= amount;
            }
            _ => panic!("Unknown direction indicator"),
        }

        dial_pos = dial_pos.rem_euclid(DIAL_SIZE);

        if dial_pos == 0 {
            output += 1;
        }
    }

    println!("Password: {}", output);
    Ok(())
}
