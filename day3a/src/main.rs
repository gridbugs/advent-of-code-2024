use std::io::{self, Read};

fn parse_stdin() -> String {
    let mut out = String::new();
    io::stdin().read_to_string(&mut out).unwrap();
    out
}

fn read_to_next_char(chars: &[char], index: usize, ch: char) -> Option<(usize, String)> {
    if index >= chars.len() {
        return None;
    }
    let mut out = String::new();
    for (i, &cur) in chars[index..].iter().enumerate() {
        if cur == ch {
            return Some((i + index, out));
        }
        out.push(cur);
    }
    None
}

fn main() {
    let input = parse_stdin();
    let input_arr = input.chars().collect::<Vec<_>>();
    let start = ['m', 'u', 'l', '('];
    let mut total = 0;
    for (index, window) in input_arr.windows(4).enumerate() {
        if window == &start {
            let (index, lhs) = if let Some((i, x)) = read_to_next_char(&input_arr, index + 4, ',') {
                if let Ok(x) = x.parse::<i32>() {
                    (i, x)
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let (_index, rhs) = if let Some((i, x)) = read_to_next_char(&input_arr, index + 1, ')')
            {
                if let Ok(x) = x.parse::<i32>() {
                    (i, x)
                } else {
                    continue;
                }
            } else {
                continue;
            };
            total += lhs * rhs;
        }
    }
    println!("{}", total);
}
