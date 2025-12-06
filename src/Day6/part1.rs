use anyhow::Result;
use std::io::stdin;

enum Operation {
    Add,
    Muultiply,
}

fn main() -> Result<()> {
    let mut input: Vec<Vec<u64>> = Vec::new();
    let mut operations: Option<Vec<Operation>> = None;

    for line in stdin().lines() {
        let line = line?;

        // opertations mode
        if line.starts_with('+') || line.starts_with('*') {
            operations = Some(
                line.split(" ")
                    .filter(|p| p.len() > 0)
                    .map(|c| match c.chars().next().unwrap() {
                        '+' => Operation::Add,
                        '*' => Operation::Muultiply,
                        _ => panic!("Unknown operand found"),
                    })
                    .collect(),
            );
        }
        // input mode
        else {
            let numbers: Vec<u64> = line
                .split(' ')
                .filter(|p| p.len() > 0)
                .map(|p| p.parse::<u64>().unwrap())
                .collect();

            // init the input holder on first line
            if input.len() == 0 {
                for _ in 0..numbers.len() {
                    input.push(Vec::new());
                }
            }

            input
                .iter_mut()
                .zip(numbers)
                .map(|(r, v)| {
                    r.push(v);
                })
                .for_each(drop);
        }
    }

    if let Some(ops) = operations {
        let out = ops
            .iter()
            .zip(input)
            .map(|(o, v)| match o {
                Operation::Add => v.iter().sum::<u64>(),
                Operation::Muultiply => v.iter().product::<u64>(),
            })
            .sum::<u64>();
        println!("Output: {}", out);
    } else {
        panic!("No operations found in input");
    }

    Ok(())
}
