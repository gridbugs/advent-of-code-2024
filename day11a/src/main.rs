use std::io;

fn parse_stdin() -> Input {
    let line = io::stdin().lines().next().unwrap().unwrap();
    let stones = line
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    Input { stones }
}

struct Input {
    stones: Vec<usize>,
}

fn split_if_even_num_digits(x: usize) -> Option<(usize, usize)> {
    let s = format!("{}", x);
    if s.len() % 2 == 0 {
        let (l, r) = s.split_at(s.len() / 2);
        Some((l.parse().unwrap(), r.parse().unwrap()))
    } else {
        None
    }
}

impl Input {
    fn step(&mut self) {
        let next = self
            .stones
            .iter()
            .flat_map(|x| {
                if *x == 0 {
                    vec![1]
                } else if let Some((l, r)) = split_if_even_num_digits(*x) {
                    vec![l, r]
                } else {
                    vec![*x * 2024]
                }
            })
            .collect::<Vec<_>>();
        self.stones = next;
    }
}

fn main() {
    let mut input = parse_stdin();
    for _ in 0..25 {
        input.step();
    }
    println!("{}", input.stones.len());
}
