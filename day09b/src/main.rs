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
    fn last_file_index_id_size_before(&self, start: usize) -> (usize, usize, usize) {
        let mut index = start - 1;
        while let Some(None) = self.blocks.get(index) {
            index -= 1;
        }
        let mut size = 0;
        let id = self.blocks[index].unwrap();
        while let Some(x) = self.blocks[index] {
            if index == 0 {
                return (0, id, size);
            }
            if x != id {
                break;
            }
            index -= 1;
            size += 1;
        }
        (index + 1, id, size)
    }

    fn next_free_index_size_after_incl(&self, mut cursor: usize) -> (usize, usize) {
        while let Some(Some(_)) = self.blocks.get(cursor) {
            cursor += 1;
        }
        let index = cursor;
        let mut size = 0;
        while let Some(None) = self.blocks.get(cursor) {
            cursor += 1;
            size += 1;
        }
        (index, size)
    }

    fn defrag(&mut self) {
        let mut file_cursor = self.blocks.len();
        while file_cursor > 0 {
            let (file_index, file_id, file_size) = self.last_file_index_id_size_before(file_cursor);
            file_cursor = file_index;
            let mut free_cursor = 0;
            loop {
                let (free_index, free_size) = self.next_free_index_size_after_incl(free_cursor);
                if free_index >= file_index {
                    break;
                }
                if free_size >= file_size {
                    for i in 0..file_size {
                        self.blocks[free_index + i] = Some(file_id);
                        self.blocks[file_index + i] = None;
                    }
                    break;
                }
                free_cursor = free_index + free_size;
            }
        }
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, id)| i * id.unwrap_or(0))
            .sum()
    }

    fn _print(&self) {
        for x in &self.blocks {
            if let Some(x) = x {
                print!("{}", x);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let mut input = parse_stdin();
    input.defrag();
    println!("{:?}", input.checksum());
}
