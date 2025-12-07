use anyhow::Result;
use std::{collections::HashSet, io::stdin};

enum Tile {
    Empty,
    Splitter,
}

fn main() -> Result<()> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut tachyon_pos_1 = HashSet::new();

    for line in stdin().lines() {
        let line = line?;
        map.push(
            line.char_indices()
                .map(|(i, c)| match c {
                    '.' => Tile::Empty,
                    '^' => Tile::Splitter,
                    'S' => {
                        tachyon_pos_1.insert(i);
                        Tile::Empty
                    }
                    _ => panic!("Unknown map tile"),
                })
                .collect(),
        );
    }

    let mut tachyon_pos_2 = HashSet::new();

    let mut out = 0u64;
    for (i, column) in map.iter().enumerate() {
        let (read_set, write_set) = if i % 2 == 0 {
            (&mut tachyon_pos_1, &mut tachyon_pos_2)
        } else {
            (&mut tachyon_pos_2, &mut tachyon_pos_1)
        };
        for pos in read_set.drain() {
            match column.iter().nth(pos).unwrap() {
                Tile::Empty => {
                    write_set.insert(pos);
                }
                Tile::Splitter => {
                    write_set.insert(pos - 1);
                    write_set.insert(pos + 1);
                    out += 1;
                }
            }
        }
    }
    println!("Output {}", out);

    Ok(())
}
