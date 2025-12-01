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
        let amount = *(&line[1..].parse::<i32>()?);

        match direction {
            'R' => {
                dial_pos += amount;
                output += dial_pos / DIAL_SIZE;
            }
            'L' => {
                output += (amount + (DIAL_SIZE - dial_pos) - 1) / DIAL_SIZE;
                dial_pos -= amount;
            }
            _ => panic!("Unknown direction indicator"),
        }

        dial_pos = dial_pos.rem_euclid(DIAL_SIZE);
    }

    println!("Password: {}", output);
    Ok(())
}
