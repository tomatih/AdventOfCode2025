use anyhow::Result;
use std::{collections::HashSet, io::stdin};

fn get_factors(val: u64) -> HashSet<u64> {
    let mut out = HashSet::new();

    if val < 2 {
        out.insert(val);
    }

    for i in 1..=val / 2 {
        if val % i == 0 {
            out.insert(i);
            out.insert(val / i);
        }
    }

    out.remove(&val); // remove self as that would not allow for any repetitions

    return out;
}

fn make_value(pattern: u64, len: u64, size: u64) -> u64 {
    let mut out = 0u64;

    for i in (0..len).step_by(size as usize) {
        out += pattern * 10u64.pow(i as u32);
    }

    return out;
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let input = &input[..input.len() - 1]; // strip the newline

    let mut output = 0u64;

    let mut out_set = HashSet::new();
    for range in input.split(',') {
        let mut bounds = range.split('-');
        let lower_bound = bounds.next().unwrap();
        let upper_bound = bounds.next().unwrap();
        print!("{} -> {}: ", lower_bound, upper_bound);

        if upper_bound.len() - lower_bound.len() > 1 {
            panic!("Very big range");
        }

        let pattern_lf = lower_bound.parse::<u64>()?;
        let pattern_uf = upper_bound.parse::<u64>()?;
        let full_range = pattern_lf..=pattern_uf;

        let factors_lower = get_factors(lower_bound.len() as u64);
        let factors_upper = get_factors(upper_bound.len() as u64);

        for len in factors_lower.union(&factors_upper) {
            let guessing_range = 10u64.pow(*len as u32 - 1)..10u64.pow(*len as u32);

            for guess in guessing_range {
                let val1 = make_value(guess, lower_bound.len() as u64, *len);
                let val2 = make_value(guess, upper_bound.len() as u64, *len);

                if full_range.contains(&val1) && val1 > 10 {
                    out_set.insert(val1);
                }
                if full_range.contains(&val2) && val2 > 10 {
                    out_set.insert(val2);
                }
            }
        }

        print!("{:?}", out_set);
        output += out_set.iter().sum::<u64>();
        out_set.clear();
        println!();
    }

    println!("Password: {}", output);

    Ok(())
}
