use std::io;

fn parse_stdin() -> Input {
    let mut rows = Vec::new();
    for line in io::stdin().lines() {
        rows.push(line.unwrap().chars().collect());
    }
    Input { rows }
}

struct Input {
    rows: Vec<Vec<char>>,
}

impl Input {
    fn count_x_mas(&self) -> usize {
        let mut count = 0;
        for i in 1..(self.rows.len() - 1) {
            for j in 1..(self.rows.len() - 1) {
                if self.rows[i][j] == 'A' {
                    let tl = self.rows[i - 1][j - 1];
                    let tr = self.rows[i - 1][j + 1];
                    let bl = self.rows[i + 1][j - 1];
                    let br = self.rows[i + 1][j + 1];
                    if !((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M')) {
                        continue;
                    }
                    if !((bl == 'M' && tr == 'S') || (bl == 'S' && tr == 'M')) {
                        continue;
                    }
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let input = parse_stdin();
    println!("{}", input.count_x_mas());
}
