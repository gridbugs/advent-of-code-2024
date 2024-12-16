use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    fn dot(&self, other: Self) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    fn mag(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    fn angle_between_rad(&self, other: Self) -> f64 {
        let cos_theta = self.dot(other) / (self.mag() * other.mag());
        cos_theta.acos()
    }

    fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn scalar_mul(&self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }

    fn grad(&self) -> f64 {
        self.y / self.x
    }
}

fn parse_button(s: &str) -> Vector2 {
    let s = s.split_once(": ").unwrap().1;
    let (xs, ys) = s.split_once(", ").unwrap();
    let x = xs.split_once("+").unwrap().1.parse().unwrap();
    let y = ys.split_once("+").unwrap().1.parse().unwrap();
    Vector2 { x, y }
}

fn parse_prize(s: &str) -> Vector2 {
    let s = s.split_once(": ").unwrap().1;
    let (xs, ys) = s.split_once(", ").unwrap();
    let offset = 10000000000000f64;
    let x = xs.split_once("=").unwrap().1.parse::<f64>().unwrap() + offset;
    let y = ys.split_once("=").unwrap().1.parse::<f64>().unwrap() + offset;
    Vector2 { x, y }
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
    a: Vector2,
    b: Vector2,
    prize: Vector2,
}

#[derive(Default, Debug)]
struct Input {
    machines: Vec<Machine>,
}

fn compute_muls(first: Vector2, second: Vector2, prize: Vector2) -> Option<(f64, f64)> {
    let prize_origin_second_rad = second.angle_between_rad(prize);
    let prize_origin_rad = first.angle_between_rad(prize);
    let unknown_point_angle = std::f64::consts::PI - (prize_origin_second_rad + prize_origin_rad);
    let sine_ratio = prize.mag() / unknown_point_angle.sin();
    let unknown_point_mag = prize_origin_rad.sin() * sine_ratio;
    let unknown_point_to_prize_a_mag = prize_origin_second_rad.sin() * sine_ratio;
    let second_mul = (unknown_point_mag / second.mag()).round();
    let first_mul = (unknown_point_to_prize_a_mag / first.mag()).round();
    if first
        .scalar_mul(first_mul)
        .add(second.scalar_mul(second_mul))
        == prize
    {
        Some((first_mul, second_mul))
    } else {
        None
    }
}

impl Machine {
    fn solve(&self) -> Option<usize> {
        assert!(self.a.grad() != self.b.grad());
        let mut cost = None;
        if let Some((a_mul, b_mul)) = compute_muls(self.a, self.b, self.prize) {
            let this_cost = ((a_mul * 3.) + b_mul) as usize;
            cost = Some(this_cost);
        }
        if let Some((b_mul, a_mul)) = compute_muls(self.b, self.a, self.prize) {
            let this_cost = ((a_mul * 3.) + b_mul) as usize;
            cost = if let Some(cost) = cost {
                Some(cost.min(this_cost))
            } else {
                Some(this_cost)
            }
        }
        cost
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
