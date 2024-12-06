use direction::Direction;
use grid_2d::{Coord, Grid, Size};
use std::{collections::HashSet, io};

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

#[derive(Clone)]
struct Input {
    start: Coord,
    map: Grid<bool>,
}

impl Input {
    fn path(&self) -> Vec<Coord> {
        let mut ret = Vec::new();
        let mut guard_coord = self.start;
        let mut facing = Direction::North;
        loop {
            let next_coord = guard_coord + facing.coord();
            match self.map.get(next_coord) {
                None => break,
                Some(true) => facing = facing.right90(),
                Some(false) => {
                    ret.push(next_coord);
                    guard_coord = next_coord;
                }
            }
        }
        ret
    }

    fn is_loop(&self) -> bool {
        let mut states: HashSet<(Coord, Direction)> = HashSet::new();
        let mut guard_coord = self.start;
        let mut facing = Direction::North;
        loop {
            if !states.insert((guard_coord, facing)) {
                break true;
            }
            let next_coord = guard_coord + facing.coord();
            match self.map.get(next_coord) {
                None => break false,
                Some(true) => facing = facing.right90(),
                Some(false) => {
                    guard_coord = next_coord;
                }
            }
        }
    }
}

fn main() {
    let input = parse_stdin();
    let mut sol = 0;
    for path_coord in input.path().into_iter().collect::<HashSet<_>>() {
        let mut with_obstruction = input.clone();
        *with_obstruction.map.get_checked_mut(path_coord) = true;
        if with_obstruction.is_loop() {
            sol += 1;
        }
    }
    println!("{}", sol);
}
