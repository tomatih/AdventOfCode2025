use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let input = &input[..input.len() - 1]; // strip the newline

    let mut output = 0u64;

    let mut tmp_str = String::new(); // reusable scratch buffer
    for range in input.split(',') {
        let mut bounds = range.split('-');
        let mut lower_bound = bounds.next().unwrap();
        let mut upper_bound = bounds.next().unwrap();
        print!("{} -> {}: ", lower_bound, upper_bound);

        // single width range with no possible pattern
        if lower_bound.len() % 2 == 1 && lower_bound.len() == upper_bound.len() {
            println!("skipped");
            continue;
        }

        if upper_bound.len() - lower_bound.len() > 1 {
            panic!("Very big range");
        }

        let pattern_lf = lower_bound.parse::<u64>()?;
        let pattern_uf = upper_bound.parse::<u64>()?;
        let full_range = pattern_lf..=pattern_uf;

        let search_len = if lower_bound.len() == upper_bound.len() {
            lower_bound.len() / 2
        } else if lower_bound.len() % 2 == 0 {
            tmp_str.clear();
            for _ in 0..lower_bound.len() {
                tmp_str.push('9');
            }
            upper_bound = tmp_str.as_str();

            lower_bound.len() / 2
        } else if upper_bound.len() % 2 == 0 {
            tmp_str.clear();
            tmp_str.push('1');
            for _ in 0..(upper_bound.len() - 1) {
                tmp_str.push('0');
            }
            lower_bound = tmp_str.as_str();

            upper_bound.len() / 2
        } else {
            panic!();
        };

        let pattern_l = lower_bound[0..search_len].parse::<u64>()?;
        let pattern_u = upper_bound[0..search_len].parse::<u64>()?;

        if pattern_l == pattern_u {
            let pattern_l2 = lower_bound[search_len..].parse::<u64>()?;
            let pattern_u2 = upper_bound[search_len..].parse::<u64>()?;
            let pattern_range2 = pattern_l2..=pattern_u2;

            if pattern_range2.contains(&pattern_l) {
                let out_val = pattern_l * 10u64.pow(search_len as u32) + pattern_l;
                output += out_val;
                print!("{}, ", out_val);
            }
        } else {
            for pattern_val in pattern_l..=pattern_u {
                let out_val = pattern_val * 10u64.pow(search_len as u32) + pattern_val;
                if full_range.contains(&out_val) {
                    output += out_val;
                    print!("{}, ", out_val);
                }
            }
        }

        println!();
    }

    println!("Password: {}", output);

    Ok(())
}
