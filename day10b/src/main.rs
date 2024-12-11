use direction::DirectionsCardinal;
use grid_2d::{Coord, Grid, Size};
use std::io;

fn parse_stdin() -> Input {
    let lines = io::stdin().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut grid = Grid::new_copy(Size::new(lines[0].len() as u32, lines.len() as u32), 0);
    for (i, line) in lines.into_iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            *grid.get_checked_mut(Coord::new(j as i32, i as i32)) =
                format!("{}", ch).parse().unwrap();
        }
    }
    Input { grid }
}

struct Input {
    grid: Grid<usize>,
}

impl Input {
    fn count_trail_ends(&self, start: Coord) -> usize {
        let mut to_visit = vec![start];
        let mut count = 0;
        while let Some(next) = to_visit.pop() {
            let neighbour_value = self.grid.get_checked(next) + 1;
            if neighbour_value == 10 {
                count += 1;
                continue;
            }
            for d in DirectionsCardinal {
                let neighbour_coord = next + d.coord();
                if let Some(value) = self.grid.get(neighbour_coord) {
                    if *value == neighbour_value {
                        to_visit.push(neighbour_coord);
                    }
                }
            }
        }
        count
    }

    fn count_all_trail_ends(&self) -> usize {
        let mut sum = 0;
        for (coord, x) in self.grid.enumerate() {
            if *x == 0 {
                sum += self.count_trail_ends(coord);
            }
        }
        sum
    }
}

fn main() {
    let input = parse_stdin();
    println!("{}", input.count_all_trail_ends());
}
