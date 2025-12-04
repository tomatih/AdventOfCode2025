use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let mut out = 0u64;

    let mut world_map: Vec<Vec<u8>> = Vec::new();
    for line in stdin().lines() {
        let line = line?;
        world_map.push(
            line.chars()
                .map(|c| match c {
                    '.' => 0u8,
                    '@' => 1u8,
                    _ => panic!("Unknown character"),
                })
                .collect(),
        );
    }

    let mut local_accum = Vec::new();
    local_accum.push((0, 0)); // dummy so that the first loop runs

    while local_accum.len() != 0 {
        local_accum.clear();
        for y in 0..world_map.len() {
            for x in 0..world_map[y].len() {
                let mut accum = 0u8;

                if world_map[y][x] == 0 {
                    continue;
                }

                for dy in -1isize..=1 {
                    for dx in -1isize..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        if let Some(row) = world_map.get((y as isize + dy) as usize) {
                            if let Some(val) = row.get((x as isize + dx) as usize) {
                                accum += val;
                            }
                        }
                    }
                }

                if accum < 4 {
                    local_accum.push((y, x));
                }
            }
        }

        out += local_accum.len() as u64;

        for (y, x) in local_accum.iter() {
            world_map[*y][*x] = 0;
        }
    }

    println!("Accesible rolls: {}", out);
    Ok(())
}
