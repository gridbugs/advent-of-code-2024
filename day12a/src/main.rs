use direction::DirectionsCardinal;
use grid_2d::{Coord, Grid, Size};
use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn parse_stdin() -> Input {
    let lines = io::stdin().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut grid = Grid::new_copy(Size::new(lines[0].len() as u32, lines.len() as u32), 'a');
    for (i, line) in lines.into_iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            *grid.get_checked_mut(Coord::new(j as i32, i as i32)) = ch;
        }
    }
    Input { grid }
}

struct Input {
    grid: Grid<char>,
}

#[derive(Debug, Default)]
struct Region {
    coords: HashSet<Coord>,
}

impl Input {
    fn regions(&self) -> Vec<Region> {
        let mut ret = Vec::new();
        let mut seen = HashSet::new();
        for (coord, ch) in self.grid.enumerate() {
            if !seen.insert(coord) {
                continue;
            }
            let mut to_visit = VecDeque::new();
            let mut region = Region::default();
            region.coords.insert(coord);
            to_visit.push_back(coord);
            while let Some(coord) = to_visit.pop_front() {
                for d in DirectionsCardinal {
                    let neighbour_coord = coord + d.coord();
                    if let Some(neighbour_ch) = self.grid.get(neighbour_coord) {
                        if neighbour_ch == ch && seen.insert(neighbour_coord) {
                            to_visit.push_back(neighbour_coord);
                            region.coords.insert(neighbour_coord);
                        }
                    }
                }
            }
            ret.push(region);
        }
        ret
    }
}

impl Region {
    fn area(&self) -> usize {
        self.coords.len()
    }

    fn perimiter(&self) -> usize {
        self.coords
            .iter()
            .map(|coord| {
                DirectionsCardinal
                    .into_iter()
                    .filter(|d| !self.coords.contains(&(coord + d.coord())))
                    .count()
            })
            .sum()
    }

    fn cost(&self) -> usize {
        self.area() * self.perimiter()
    }
}

fn main() {
    let input = parse_stdin();
    let mut total = 0;
    let regions = input.regions();
    for region in regions {
        total += region.cost();
    }
    println!("{}", total);
}
