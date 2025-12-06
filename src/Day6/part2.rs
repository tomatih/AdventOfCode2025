use anyhow::Result;
use std::{cmp::max, io::stdin};

enum Operation {
    Add,
    Muultiply,
}

fn main() -> Result<()> {
    let mut input: Vec<String> = Vec::new();
    let mut operations: Option<Vec<Operation>> = None;
    let mut max_len = 0;

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
            max_len = max(max_len, line.len());
            input.push(line);
        }
    }

    // read the columns
    let mut numbers: Vec<u64> = Vec::new();
    let opt = operations.unwrap();
    let mut opt_iter = opt.iter();
    let mut total = 0u64;

    for i in 0..=max_len {
        let number = input
            .iter()
            .map(|row| row.chars().nth(i).unwrap_or(' '))
            .filter(|c| *c != ' ')
            .collect::<String>()
            .parse::<u64>();

        match number {
            Ok(v) => numbers.push(v),
            Err(_) => {
                total += match opt_iter.next().unwrap() {
                    Operation::Add => numbers.iter().sum::<u64>(),
                    Operation::Muultiply => numbers.iter().product(),
                };

                numbers.clear();
            }
        }
    }
    println!("Out: {}", total);

    Ok(())
}
