use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let mut out = 0u64;

    for line in stdin().lines() {
        let line = line?;

        let mut max_1 = -1i8;
        let mut max_2 = -1i8;
        for (i, char) in line.char_indices() {
            let val = char.to_digit(10).unwrap() as i8;

            if val > max_1 && (i + 1) < line.len() {
                max_1 = val;
                max_2 = -1;
            } else if val > max_2 {
                max_2 = val;
            }
        }
        out += max_1 as u64 * 10u64 + max_2 as u64;
    }

    println!("Maximum joltage: {}", out);
    Ok(())
}
