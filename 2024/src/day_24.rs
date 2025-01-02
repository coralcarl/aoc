#![allow(unused_variables)]

use std::collections::BTreeMap;

#[derive(Debug)]
struct WireNew<'a> {
    name: &'a str,
    value: Option<bool>,
}

#[derive(Debug)]
struct GateNew {
    input1_id: usize,
    input2_id: usize,
    output_id: usize,
    logic: fn(&bool, &bool) -> bool,
}

#[derive(Debug)]
struct AdderNew<'a> {
    wires: Vec<WireNew<'a>>,
    gates: Vec<GateNew>,
    output_ids: Vec<usize>,
}

impl<'a> AdderNew<'a> {
    fn and(input1: &bool, input2: &bool) -> bool {
        *input1 && *input2
    }

    fn or(input1: &bool, input2: &bool) -> bool {
        *input1 || *input2
    }

    fn xor(input1: &bool, input2: &bool) -> bool {
        *input1 ^ *input2
    }

    fn from_str(input: &'a str) -> Self {
        let mut wires = Vec::new();
        let mut gates = Vec::new();

        let mut wire_map = BTreeMap::new();

        let mut lines = input.lines();

        while let Some((name, value)) = lines.next().unwrap().split_once(": ") {
            wires.push(WireNew {
                name,
                value: Some(value != "0"),
            });
            wire_map.insert(name, wires.len() - 1);
        }

        while let Some(line) = lines.next() {
            let (left, output_name) = line.split_once(" -> ").unwrap();
            let mut parts = left.split(" ");
            let input1_name = parts.next().unwrap();
            let logic = parts.next().unwrap().chars().next().unwrap();
            let input2_name = parts.next().unwrap();

            let input1_id = *wire_map.entry(input1_name).or_insert_with(|| {
                wires.push(WireNew {
                    name: input1_name,
                    value: None,
                });
                wires.len() - 1
            });
            let input2_id = *wire_map.entry(input2_name).or_insert_with(|| {
                wires.push(WireNew {
                    name: input2_name,
                    value: None,
                });
                wires.len() - 1
            });
            let output_id = *wire_map.entry(output_name).or_insert_with(|| {
                wires.push(WireNew {
                    name: output_name,
                    value: None,
                });
                wires.len() - 1
            });

            let logic = match logic {
                'A' => AdderNew::and,
                'O' => AdderNew::or,
                'X' => AdderNew::xor,
                _ => unreachable!(),
            };

            gates.push(GateNew {
                input1_id,
                input2_id,
                output_id,
                logic,
            });
        }

        let output_ids = wire_map
            .into_iter()
            .filter_map(|(name, id)| {
                if name.starts_with('z') {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();

        Self {
            wires,
            gates,
            output_ids,
        }
    }

    fn do_calc(&mut self) -> u64 {
        while self
            .output_ids
            .iter()
            .any(|&id| self.wires[id].value.is_none())
        {
            for gate in &self.gates {
                if self.wires[gate.output_id].value.is_some() {
                    continue;
                }

                if let Some(input1) = self.wires[gate.input1_id].value.as_ref() {
                    if let Some(input2) = self.wires[gate.input2_id].value.as_ref() {
                        self.wires[gate.output_id].value = Some((gate.logic)(input1, input2));
                    }
                }
            }
        }

        let mut result = 0;
        let mut factor = 1;
        for digit in self
            .output_ids
            .iter()
            .map(|id| self.wires[*id].value.unwrap())
        {
            if digit {
                result += factor;
            }
            factor *= 2;
        }
        result
    }

    fn find_bad_gates(&self) -> Vec<&'a str> {
        let mut bad_gates = Vec::new();

        for gate in &self.gates {
            let is_output = self.wires[gate.output_id].name.starts_with('z');
            if is_output
                && gate.logic != AdderNew::xor
                && gate.output_id != self.output_ids[self.output_ids.len() - 1]
            {
                bad_gates.push(self.wires[gate.output_id].name);
            }

            let is_input_for = self
                .gates
                .iter()
                .filter(|other| {
                    other.input1_id == gate.output_id || other.input2_id == gate.output_id
                })
                .collect::<Vec<_>>();

            if gate.logic == AdderNew::and
                && is_input_for.iter().any(|other| other.logic != AdderNew::or)
                && self.wires[gate.input1_id].name != "x00"
                && self.wires[gate.input1_id].name != "y00"
            {
                bad_gates.push(self.wires[gate.output_id].name);
            }

            if gate.logic == AdderNew::xor
                && self.output_ids[self.output_ids.len() - 1] != gate.output_id
                && (is_input_for.iter().any(|other| other.logic == AdderNew::or)
                    || (is_output && self.wires[gate.input1_id].name.starts_with(&['x', 'y']))
                    || (!is_output && !self.wires[gate.input1_id].name.starts_with(&['x', 'y'])))
            {
                bad_gates.push(self.wires[gate.output_id].name);
            }
        }
        bad_gates
    }
}

pub fn part1(input: &str) -> String {
    let mut adder = AdderNew::from_str(input);
    adder.do_calc().to_string()
}

pub fn part2(input: &str) -> String {
    let adder = AdderNew::from_str(input);
    let mut bad_outputs = adder.find_bad_gates();
    bad_outputs.sort();
    bad_outputs.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE1), "2024");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE1), "todo");
    }
}
