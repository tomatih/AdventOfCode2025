use anyhow::Result;
use std::{io::stdin, ops::RangeInclusive};

fn main() -> Result<()> {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();

    for line in stdin().lines() {
        let line = line?;

        // mode switch
        if line.len() == 0 {
            break;
        }

        let mut bounds = line.split('-');
        let lower_bound = bounds.next().unwrap().parse::<u64>()?;
        let upper_bound = bounds.next().unwrap().parse::<u64>()?;

        let range = lower_bound..=upper_bound;

        ranges.push(range);
    }

    ranges.sort_by_key(|r| *r.start());

    let mut i = 0;
    while i < ranges.len() {
        // ranges are mergable
        if i + 1 < ranges.len() && ranges[i + 1].start() <= ranges[i].end() {
            // next range not already fully accounted for
            if !(ranges[i + 1].end() <= ranges[i].end()) {
                ranges[i] = *ranges[i].start()..=*ranges[i + 1].end(); // merge with next one
            }
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }

    let out: u64 = ranges.iter().map(|r| r.end() - r.start() + 1).sum();
    println!("Output: {}", out);

    Ok(())
}
