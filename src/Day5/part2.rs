use anyhow::Result;
use std::{io::stdin, ops::RangeInclusive};

fn main() -> Result<()> {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut out = 0u64;

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
        let mut needs_adding = Some(ranges.len());

        for i in 0..ranges.len() {
            // range already fully accoubnted for
            if ranges[i].contains(&lower_bound) && ranges[i].contains(&upper_bound) {
                needs_adding = None;
                println!("Dropping {:?}", range);
                break;
            }
            // can extend a ranhe to the left
            else if ranges[i].contains(&upper_bound) {
                // check if can be merged with the previous range
                if i != 0 && ranges[i - 1].contains(&lower_bound) {
                    println!(
                        "Used {:?} to join {:?} and {:?}",
                        range,
                        ranges[i - 1],
                        ranges[i]
                    );
                    ranges[i - 1] = *ranges[i - 1].start()..=*ranges[i].end();
                    ranges.remove(i);
                } else {
                    ranges[i] = lower_bound..=*ranges[i].end();
                    println!("Merged {:?} with {:?}", range, ranges[i]);
                }
                needs_adding = None;
                break;
            }
            // can extend to the right
            else if ranges[i].contains(&lower_bound) {
                // check if can be merged with the previous range
                if i + 1 < ranges.len() && ranges[i + 1].contains(&upper_bound) {
                    println!(
                        "Used {:?} to join {:?} and {:?}",
                        range,
                        ranges[i],
                        ranges[i + 1]
                    );
                    ranges[i] = *ranges[i].start()..=*ranges[i + 1].end();
                    ranges.remove(i + 1);
                } else {
                    ranges[i] = lower_bound..=*ranges[i].end();
                    println!("Merged {:?} with {:?}", range, ranges[i]);
                }
                needs_adding = None;
                break;
            }
            // foudn insertion place for the range
            else if ranges[i].end() < &lower_bound
                && i + 1 < ranges.len()
                && ranges[i + 1].start() > &upper_bound
            {
                println!("Found an insersion spot {}", i);
                needs_adding = Some(i + 1);
                break;
            }
        }

        if let Some(i) = needs_adding {
            println!("Adding {:?} at pos {}", range, i);
            ranges.insert(i, range);
        }
    }

    for range in ranges {
        println!("{:?}", range);
        out += range.end() - range.start() + 1;
    }

    println!("Output: {}", out);

    Ok(())
}

// WWHAT IF NEW RANGE IS BIG ENOUGHT TO JOIN MULTIPLE RANGES!
