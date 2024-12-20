#![allow(unused_variables)]

#[derive(Clone)]
struct TBComputer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
    ipointer: usize,
    output: Vec<u8>,
}

impl TBComputer {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        Self {
            a: lines.next().unwrap()[12..].parse::<u64>().unwrap(),
            b: lines.next().unwrap()[12..].parse::<u64>().unwrap(),
            c: lines.next().unwrap()[12..].parse::<u64>().unwrap(),
            program: lines.skip(1).next().unwrap()[9..]
                .split(',')
                .map(|c| c.parse::<u8>().unwrap())
                .collect(),
            ipointer: 0,
            output: Vec::new(),
        }
    }

    fn map_combo_op(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn adv(&mut self, operand: u64) {
        self.a = self.a >> self.map_combo_op(operand);
        self.ipointer += 2;
    }

    fn bxl(&mut self, operand: u64) {
        self.b ^= operand;
        self.ipointer += 2;
    }

    fn bst(&mut self, operand: u64) {
        self.b = self.map_combo_op(operand) % 8;
        self.ipointer += 2;
    }

    fn jnz(&mut self, operand: u64) {
        if self.a != 0 {
            self.ipointer = operand as usize;
        } else {
            self.ipointer += 2;
        }
    }

    fn bxc(&mut self, operand: u64) {
        self.b ^= self.c;
        self.ipointer += 2;
    }

    fn out(&mut self, operand: u64) {
        self.output.push((self.map_combo_op(operand) % 8) as u8);
        self.ipointer += 2;
    }

    fn bdv(&mut self, operand: u64) {
        self.b = self.a >> self.map_combo_op(operand);
        self.ipointer += 2;
    }

    fn cdv(&mut self, operand: u64) {
        self.c = self.a >> self.map_combo_op(operand);
        self.ipointer += 2;
    }

    fn run(&mut self) -> Vec<u8> {
        while self.ipointer < self.program.len() {
            let opcode = self.program[self.ipointer];
            let operand = self.program[self.ipointer + 1];

            match opcode {
                0 => self.adv(operand.into()),
                1 => self.bxl(operand.into()),
                2 => self.bst(operand.into()),
                3 => self.jnz(operand.into()),
                4 => self.bxc(operand.into()),
                5 => self.out(operand.into()),
                6 => self.bdv(operand.into()),
                7 => self.cdv(operand.into()),
                _ => panic!(),
            }
        }

        self.output.clone()
    }

    fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.ipointer = 0;
        self.output.clear();
    }
}

pub fn part1(input: &str) -> String {
    let mut tbc = TBComputer::new(&input);
    tbc.run()
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part2(input: &str) -> String {
    let mut tbc = TBComputer::new(&input);

    let mut valid = Vec::new();

    for a in 1..1024 {
        tbc.reset();
        tbc.a = a;
        tbc.run();
        if tbc.output[0] == tbc.program[0] {
            valid.push(a);
        }
    }

    for pos in 1..tbc.program.len() {
        let mut valid_next = Vec::new();
        for check in &valid {
            for num in 0..8 {
                let v_new = (num << (7 + 3 * pos)) | check;
                tbc.reset();
                tbc.a = v_new;
                tbc.run();
                if tbc.output.len() > pos && tbc.output[pos] == tbc.program[pos] {
                    valid_next.push(v_new);
                }
            }
        }

        valid = valid_next;
    }

    valid.iter().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE2), "117440");
    }
}
