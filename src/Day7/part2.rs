use anyhow::Result;
use std::io::stdin;

enum Tile {
    Empty,
    Splitter,
}

#[derive(Clone, Copy)]
struct TachyonRay {
    pos: usize,
    multiplicity: u64,
}

fn main() -> Result<()> {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut tachyon_pos_1 = Vec::new();

    for line in stdin().lines() {
        let line = line?;
        map.push(
            line.char_indices()
                .map(|(i, c)| match c {
                    '.' => Tile::Empty,
                    '^' => Tile::Splitter,
                    'S' => {
                        tachyon_pos_1.push(TachyonRay {
                            pos: i,
                            multiplicity: 1,
                        });
                        Tile::Empty
                    }
                    _ => panic!("Unknown map tile"),
                })
                .collect(),
        );
    }

    let mut tachyon_pos_2 = Vec::new();

    for (i, column) in map.iter().enumerate() {
        let (read_set, write_set) = if i % 2 == 0 {
            (&mut tachyon_pos_1, &mut tachyon_pos_2)
        } else {
            (&mut tachyon_pos_2, &mut tachyon_pos_1)
        };
        for ray in read_set.iter() {
            match column.iter().nth(ray.pos).unwrap() {
                Tile::Empty => {
                    write_set.push(*ray);
                }
                Tile::Splitter => {
                    match write_set.iter_mut().find(|r| r.pos == ray.pos - 1) {
                        Some(r) => r.multiplicity += ray.multiplicity,
                        None => write_set.push(TachyonRay {
                            pos: ray.pos - 1,
                            multiplicity: ray.multiplicity,
                        }),
                    }
                    match write_set.iter_mut().find(|r| r.pos == ray.pos + 1) {
                        Some(r) => r.multiplicity += ray.multiplicity,
                        None => write_set.push(TachyonRay {
                            pos: ray.pos + 1,
                            multiplicity: ray.multiplicity,
                        }),
                    }
                }
            }
        }
        read_set.clear();
    }

    let last_layer = if tachyon_pos_1.is_empty() {
        tachyon_pos_2
    } else {
        tachyon_pos_1
    };
    let out = last_layer.iter().map(|r| r.multiplicity).sum::<u64>();
    println!("Output {}", out);

    Ok(())
}
