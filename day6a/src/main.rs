use direction::Direction;
use grid_2d::{Coord, Grid, Size};
use std::io;

fn parse_stdin() -> Input {
    let lines = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line
        })
        .collect::<Vec<_>>();
    let mut map = Grid::new_copy(Size::new(lines[0].len() as u32, lines.len() as u32), false);
    let mut start = Coord::new(0, 0);
    for (i, line) in lines.into_iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '#' => *map.get_checked_mut(Coord::new(j as i32, i as i32)) = true,
                '.' => (),
                '^' => start = Coord::new(j as i32, i as i32),
                _ => panic!(),
            }
        }
    }
    Input { start, map }
}

struct Input {
    start: Coord,
    map: Grid<bool>,
}

fn main() {
    let Input { start, map } = parse_stdin();
    let mut visited = map.map_ref(|_| false);
    let mut guard_coord = start;
    let mut facing = Direction::North;
    loop {
        *visited.get_checked_mut(guard_coord) = true;
        let next_coord = guard_coord + facing.coord();
        match map.get(next_coord) {
            None => break,
            Some(true) => facing = facing.right90(),
            Some(false) => guard_coord = next_coord,
        }
    }
    let sol = visited.iter().filter(|x| **x).count();
    println!("{}", sol);
}
