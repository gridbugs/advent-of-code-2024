use grid_2d::{Coord, Size};
use std::io;

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
    fn tick_n(&mut self, n: usize) {
        for _ in 0..n {
            self.tick();
        }
    }
    fn safety_factor(&self) -> usize {
        let mid_x = SIZE.x() as i32 / 2;
        let mid_y = SIZE.y() as i32 / 2;
        let tl = self
            .robots
            .iter()
            .filter(|r| r.pos.x < mid_x && r.pos.y < mid_y)
            .count();
        let tr = self
            .robots
            .iter()
            .filter(|r| r.pos.x > mid_x && r.pos.y < mid_y)
            .count();
        let bl = self
            .robots
            .iter()
            .filter(|r| r.pos.x < mid_x && r.pos.y > mid_y)
            .count();
        let br = self
            .robots
            .iter()
            .filter(|r| r.pos.x > mid_x && r.pos.y > mid_y)
            .count();
        tl * tr * bl * br
    }
}

fn main() {
    let mut input = Input::parse_stdin();
    input.tick_n(100);
    println!("{}", input.safety_factor());
}
