use std::{collections::HashMap, io};

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

fn step(x: usize) -> Vec<usize> {
    if x == 0 {
        vec![1]
    } else if let Some((l, r)) = split_if_even_num_digits(x) {
        vec![l, r]
    } else {
        vec![x * 2024]
    }
}

struct State {
    counts: HashMap<usize, usize>,
}

impl State {
    fn new(stones: &[usize]) -> Self {
        let mut counts = HashMap::new();
        for stone in stones {
            *counts.entry(*stone).or_default() += 1;
        }
        Self { counts }
    }

    fn step(&mut self) {
        let mut counts = HashMap::new();
        for (stone, count) in self.counts.iter() {
            let next_stones = step(*stone);
            for next_stone in next_stones {
                *counts.entry(next_stone).or_default() += count;
            }
        }
        self.counts = counts;
    }

    fn count(&self) -> usize {
        self.counts.values().copied().sum()
    }
}

fn main() {
    let input = parse_stdin();
    let mut state = State::new(&input.stones);
    for _ in 0..75 {
        state.step();
    }
    println!("{}", state.count());
}
