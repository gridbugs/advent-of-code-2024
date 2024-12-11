use grid_2d::{Coord, Size};
use std::{
    collections::{HashMap, HashSet},
    io,
};

fn parse_stdin() -> Input {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    let mut tower_coords: HashMap<char, Vec<Coord>> = HashMap::new();
    let size = Size::new(lines[0].len() as u32, lines.len() as u32);
    for (i, line) in lines.into_iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            let coord = Coord::new(j as i32, i as i32);
            match ch {
                '.' => (),
                c => tower_coords.entry(c).or_default().push(coord),
            }
        }
    }
    Input { size, tower_coords }
}

struct Input {
    size: Size,
    tower_coords: HashMap<char, Vec<Coord>>,
}

fn antinodes(a: Coord, b: Coord) -> (Coord, Coord) {
    let delta = a - b;
    (a + delta, b - delta)
}

fn main() {
    let Input { size, tower_coords } = parse_stdin();
    let mut all_antinodes = Vec::new();
    for coords in tower_coords.values() {
        for a in coords {
            for b in coords {
                if a != b {
                    let (antinode_a, antinode_b) = antinodes(*a, *b);
                    all_antinodes.push(antinode_a);
                    all_antinodes.push(antinode_b);
                }
            }
        }
    }
    let antinodes = all_antinodes
        .into_iter()
        .filter(|x| x.is_valid(size))
        .collect::<HashSet<_>>();
    println!("{}", antinodes.len());
}
