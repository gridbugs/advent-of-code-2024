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
    fn rows(&self) -> impl Iterator<Item = String> + '_ {
        self.rows
            .iter()
            .map(|chars| chars.iter().cloned().collect())
    }
    fn cols(&self) -> impl Iterator<Item = String> + '_ {
        let mut cols = Vec::new();
        for i in 0..self.rows[0].len() {
            let mut col = String::new();
            for row in self.rows.iter() {
                col.push(row[i]);
            }
            cols.push(col);
        }
        cols.into_iter()
    }
    fn diags(&self) -> impl Iterator<Item = String> + '_ {
        let mut diags = Vec::new();
        for i in 0..self.rows.len() {
            let mut diag = String::new();
            let mut x = 0;
            let mut y = i as i32;
            while y >= 0 {
                diag.push(self.rows[y as usize][x]);
                x += 1;
                y -= 1;
            }
            diags.push(diag);
            let mut diag = String::new();
            let mut x = (self.rows.len() - 1) as i32;
            let mut y = i as i32;
            while y >= 0 {
                diag.push(self.rows[y as usize][x as usize]);
                x -= 1;
                y -= 1;
            }
            diags.push(diag);
            if i > 0 {
                let mut diag = String::new();
                let mut x = 0;
                let mut y = i;
                while y < self.rows.len() {
                    diag.push(self.rows[y as usize][x as usize]);
                    x += 1;
                    y += 1;
                }
                diags.push(diag);
                let mut diag = String::new();
                let mut x = (self.rows.len() - 1) as i32;
                let mut y = i;
                while y < self.rows.len() {
                    diag.push(self.rows[y as usize][x as usize]);
                    x -= 1;
                    y += 1;
                }
                diags.push(diag);
            }
        }
        diags.into_iter()
    }
    fn all_fwd(&self) -> impl Iterator<Item = String> + '_ {
        self.rows().chain(self.cols()).chain(self.diags())
    }
    fn all(&self) -> impl Iterator<Item = String> + '_ {
        self.all_fwd()
            .chain(self.all_fwd().map(|s| s.chars().rev().collect()))
    }
}

fn main() {
    let input = parse_stdin();
    let mut count = 0;
    for d in input.all() {
        let arr = d.chars().collect::<Vec<_>>();
        for w in arr.windows(4) {
            if w == &['X', 'M', 'A', 'S'] {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
