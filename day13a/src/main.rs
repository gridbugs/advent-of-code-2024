use direction::DirectionsCardinal;
use grid_2d::{Coord, Grid, Size};
use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn parse_button(s: &str) -> Coord {
    let s = s.split_once(": ").unwrap().1;
    let (xs, ys) = s.split_once(", ").unwrap();
    let x = xs.split_once("+").unwrap().1.parse().unwrap();
    let y = ys.split_once("+").unwrap().1.parse().unwrap();
    Coord { x, y }
}

fn parse_prize(s: &str) -> Coord {
    let s = s.split_once(": ").unwrap().1;
    let (xs, ys) = s.split_once(", ").unwrap();
    let x = xs.split_once("=").unwrap().1.parse().unwrap();
    let y = ys.split_once("=").unwrap().1.parse().unwrap();
    Coord { x, y }
}

fn parse_stdin() -> Input {
    let lines = io::stdin().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut input = Input::default();
    for block in lines.chunks(4) {
        let a = parse_button(block[0].as_str());
        let b = parse_button(block[1].as_str());
        let prize = parse_prize(block[2].as_str());
        input.machines.push(Machine { a, b, prize });
    }
    input
}

#[derive(Debug)]
struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

#[derive(Default, Debug)]
struct Input {
    machines: Vec<Machine>,
}

impl Machine {
    fn solve(&self) -> Option<usize> {
        let mut best = None;
        let mut lowest_cost = usize::MAX;
        for i in 0..100 {
            for j in 0..100 {
                let pos = (self.a * i) + (self.b * j);
                if pos == self.prize {
                    let cost = ((i * 3) + j) as usize;
                    if cost < lowest_cost {
                        lowest_cost = cost;
                        best = Some(cost);
                    }
                }
            }
        }
        best
    }
}

fn main() {
    let input = parse_stdin();
    let mut total = 0;
    for machine in input.machines {
        total += machine.solve().unwrap_or(0);
    }
    println!("{}", total);
}
