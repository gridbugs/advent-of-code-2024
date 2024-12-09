use std::io;

fn parse_stdin() -> Input {
    let line = io::stdin().lines().next().unwrap().unwrap();
    let mut chars = line.chars();
    let mut blocks = Vec::new();
    let mut id = 0;
    loop {
        let ch = chars.next().unwrap();
        let num = format!("{}", ch).parse::<usize>().unwrap();
        for _ in 0..num {
            blocks.push(Some(id));
        }
        id += 1;
        if let Some(ch) = chars.next() {
            let num = format!("{}", ch).parse::<usize>().unwrap();
            for _ in 0..num {
                blocks.push(None);
            }
        } else {
            break;
        }
    }
    Input { blocks }
}

struct Input {
    blocks: Vec<Option<usize>>,
}

impl Input {
    fn defrag(&mut self) {
        let mut cursor = 0;
        loop {
            while let Some(Some(_)) = self.blocks.get(cursor) {
                cursor += 1;
            }
            while self.blocks.last().unwrap().is_none() {
                self.blocks.pop();
            }
            if cursor >= self.blocks.len() {
                break;
            }
            let last_index = self.blocks.len() - 1;
            self.blocks.swap(cursor, last_index);
            assert!(self.blocks[cursor].is_some());
        }
        for x in &self.blocks {
            assert!(x.is_some());
        }
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, id)| i * id.unwrap_or(0))
            .sum()
    }
}

fn main() {
    let mut input = parse_stdin();
    input.defrag();
    println!("{:?}", input.checksum());
}
