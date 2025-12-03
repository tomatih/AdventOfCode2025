use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let mut out = 0u64;

    for line in stdin().lines() {
        let line = line?;

        let mut max_arr = [-1i8; 12];

        for (i, char) in line.char_indices() {
            let val = char.to_digit(10).unwrap() as i8;

            for (j, max) in max_arr.iter().enumerate() {
                if val > *max && (i + max_arr.len() - j - 1) < line.len() {
                    max_arr[j] = val;
                    for k in (j + 1)..max_arr.len() {
                        max_arr[k] = -1;
                    }
                    break;
                }
            }
        }

        out += max_arr
            .iter()
            .enumerate()
            .map(|(i, v)| 10u64.pow(max_arr.len() as u32 - i as u32 - 1u32) * *v as u64)
            .sum::<u64>();
    }

    println!("Maximum joltage: {}", out);
    Ok(())
}
