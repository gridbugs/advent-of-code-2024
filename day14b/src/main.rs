use direction::DirectionsCardinal;
use grid_2d::{Coord, Grid, Size};
use std::{collections::HashSet, io};

#[derive(Debug)]
struct Robot {
    pos: Coord,
    vel: Coord,
}

//const SIZE: Size = Size::new_u16(11, 7);
const SIZE: Size = Size::new_u16(101, 103);

impl Robot {
    fn parse_coord(s: &str) -> Coord {
        let (_, c) = s.split_once("=").unwrap();
        let (x, y) = c.split_once(",").unwrap();
        Coord {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
    fn parse(s: &str) -> Self {
        let (p, v) = s.split_once(" ").unwrap();
        let pos = Self::parse_coord(p);
        let vel = Self::parse_coord(v);
        Self { pos, vel }
    }
    fn tick(&mut self) {
        let mut next = self.pos + self.vel;
        while next.x < 0 {
            next.x += SIZE.x() as i32;
        }
        while next.x >= SIZE.x() as i32 {
            next.x -= SIZE.x() as i32;
        }
        while next.y < 0 {
            next.y += SIZE.y() as i32;
        }
        while next.y >= SIZE.y() as i32 {
            next.y -= SIZE.y() as i32;
        }
        self.pos = next;
    }
}

#[derive(Default, Debug)]
struct Input {
    robots: Vec<Robot>,
}

impl Input {
    fn parse_stdin() -> Self {
        let lines = io::stdin().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
        let mut s = Self::default();
        for line in lines {
            s.robots.push(Robot::parse(line.as_str()));
        }
        s
    }
    fn tick(&mut self) {
        for r in self.robots.iter_mut() {
            r.tick();
        }
    }
    fn to_count_grid(&self) -> Grid<usize> {
        let mut grid = Grid::new_copy(SIZE, 0);
        for r in &self.robots {
            *grid.get_checked_mut(r.pos) += 1;
        }
        grid
    }

    fn is_xmas_tree_candidate(&self) -> bool {
        let grid = self.to_count_grid();
        let mut seen = HashSet::new();
        for (coord, count) in grid.enumerate() {
            if *count == 0 || !seen.insert(coord) {
                continue;
            }
            let mut region_size = 0;
            let mut to_visit = vec![coord];
            while let Some(coord) = to_visit.pop() {
                for d in DirectionsCardinal {
                    let neighbour_coord = coord + d.coord();
                    if let Some(count) = grid.get(neighbour_coord) {
                        if *count > 0 && seen.insert(neighbour_coord) {
                            to_visit.push(neighbour_coord);
                            region_size += 1;
                            if region_size >= 20 {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
    fn print(&self) {
        let grid = self.to_count_grid();
        for row in grid.rows() {
            for x in row {
                if *x > 0 {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut input = Input::parse_stdin();
    for i in 0..100000 {
        if input.is_xmas_tree_candidate() {
            println!("{}", i);
            input.print();
        }
        input.tick();
    }
}
