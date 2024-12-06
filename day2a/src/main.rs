use std::io;

fn parse_stdin() -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let parts = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        out.push(parts.collect());
    }
    out
}

fn is_increasing(report: &[i32]) -> bool {
    report.windows(2).all(|cs| cs[0] < cs[1])
}

fn is_decreasing(report: &[i32]) -> bool {
    report.windows(2).all(|cs| cs[0] > cs[1])
}

fn is_safe(report: &[i32]) -> bool {
    (is_increasing(report) || is_decreasing(report))
        && (report.windows(2).all(|cs| {
            let diff = (cs[0] - cs[1]).abs();
            diff >= 1 && diff <= 3
        }))
}

fn main() {
    let reports = parse_stdin();
    let solution = reports.iter().filter(|r| is_safe(r)).count();
    println!("{}", solution);
}
