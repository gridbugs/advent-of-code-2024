use std::{collections::HashMap, io};

fn parse_stdin() -> Input {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut lines = io::stdin().lines();
    for line in &mut lines {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('|');
        rules.push(Rule {
            before: parts.next().unwrap().parse().unwrap(),
            after: parts.next().unwrap().parse().unwrap(),
        });
    }
    for line in &mut lines {
        let line = line.unwrap();
        let parts = line.split(',');
        updates.push(parts.map(|x| x.parse().unwrap()).collect());
    }
    Input { rules, updates }
}

struct Rule {
    before: usize,
    after: usize,
}

struct Input {
    rules: Vec<Rule>,
    updates: Vec<Vec<usize>>,
}

fn main() {
    let Input { rules, updates } = parse_stdin();
    let mut sol = 0;
    'outer: for updates in updates {
        let mut tab: HashMap<usize, usize> = HashMap::new();
        for (i, &update) in updates.iter().enumerate() {
            tab.insert(update, i);
        }
        for rule in &rules {
            if let Some((before_pos, after_pos)) = tab.get(&rule.before).zip(tab.get(&rule.after)) {
                if before_pos > after_pos {
                    continue 'outer;
                }
            }
        }
        sol += updates[updates.len() / 2];
    }
    println!("{}", sol);
}
