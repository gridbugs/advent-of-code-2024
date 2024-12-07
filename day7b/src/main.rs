use std::io;

fn parse_stdin() -> Input {
    let mut equations = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut parts = line.split(':');
        let value = parts.next().unwrap().parse::<i64>().unwrap();
        let mut operands = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        operands.reverse();
        equations.push(Equation { value, operands });
    }
    Input { equations }
}

#[derive(Clone, Debug)]
struct Equation {
    value: i64,
    operands: Vec<i64>,
}

impl Equation {
    fn can_satisfy(&self) -> bool {
        assert!(self.operands.len() > 0);
        if self.operands.len() == 1 {
            self.operands[0] == self.value
        } else {
            let mut next = self.clone();
            let lhs = next.operands.pop().unwrap();
            let rhs = next.operands.pop().unwrap();
            let mut next_add = next;
            let mut next_mul = next_add.clone();
            let mut next_cat = next_add.clone();
            next_add.operands.push(lhs + rhs);
            next_mul.operands.push(lhs * rhs);
            next_cat
                .operands
                .push(format!("{}{}", lhs, rhs).parse::<i64>().unwrap());
            next_add.can_satisfy() || next_mul.can_satisfy() || next_cat.can_satisfy()
        }
    }
}

struct Input {
    equations: Vec<Equation>,
}

fn main() {
    let Input { equations } = parse_stdin();
    let mut sol = 0;
    for equation in equations {
        if equation.can_satisfy() {
            sol += equation.value;
        }
    }
    println!("{}", sol);
}
