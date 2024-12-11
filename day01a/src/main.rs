use std::io;

fn parse_stdin() -> (Vec<i32>, Vec<i32>) {
    let mut l = Vec::new();
    let mut r = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        l.push(parts.next().unwrap());
        r.push(parts.next().unwrap());
    }
    (l, r)
}

fn main() {
    let (mut l, mut r) = parse_stdin();
    l.sort();
    r.sort();
    let solution: i32 = l
        .into_iter()
        .zip(r.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
    println!("{}", solution);
}
