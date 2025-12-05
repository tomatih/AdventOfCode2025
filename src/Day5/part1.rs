use anyhow::Result;
use std::{io::stdin, ops::RangeInclusive};

fn main() -> Result<()> {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut out = 0u64;
    let mut got_all = false;

    for line in stdin().lines() {
        let line = line?;

        // mode switch
        if line.len() == 0 {
            got_all = true;
            continue;
        }

        if got_all {
            let val = line.parse::<u64>()?;

            for range in ranges.iter() {
                if range.contains(&val) {
                    out += 1;
                    break;
                }
            }
        } else {
            let mut bounds = line.split('-');
            let lower_bound = bounds.next().unwrap().parse::<u64>()?;
            let upper_bound = bounds.next().unwrap().parse::<u64>()?;

            let range = lower_bound..=upper_bound;
            ranges.push(range);
        }
    }

    println!("Output: {}", out);

    Ok(())
}
