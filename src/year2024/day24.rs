use std::collections::{HashMap, HashSet};
use std::fs;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;

pub fn part1(input:&str) -> u64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let wires_raw: Vec<&str> = parts[0].lines().collect();
    let gates_raw: Vec<&str> = parts[1].lines().collect();

    let mut wires: HashMap<String, Option<u32>> = HashMap::new();
    for line in wires_raw {
        let parts: Vec<&str> = line.split(": ").collect();
        wires.insert(parts[0].to_string(), Some(parts[1].parse().unwrap()));
    }

    let mut gates: Vec<crate::year2024::day24::Gate> = Vec::new();
    for line in gates_raw {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let inputs: Vec<&str> = parts[0].split_whitespace().collect();
        let gate = crate::year2024::day24::Gate {
            a: inputs[0].to_string(),
            op: match inputs[1] {
                "AND" => crate::year2024::day24::Op::AND,
                "OR" => crate::year2024::day24::Op::OR,
                "XOR" => crate::year2024::day24::Op::XOR,
                _ => panic!("Unknown operation"),
            },
            b: inputs[2].to_string(),
            output: parts[1].to_string(),
        };
        gates.push(gate);

        if !wires.contains_key(inputs[0]) {
            wires.insert(inputs[0].to_string(), None);
        }
        if !wires.contains_key(inputs[2]) {
            wires.insert(inputs[2].to_string(), None);
        }
        if !wires.contains_key(parts[1]) {
            wires.insert(parts[1].to_string(), None);
        }
    }

    loop {
        let mut progress = false;
        for gate in &gates {
            if let (Some(a_val), Some(b_val)) = (wires[&gate.a], wires[&gate.b]) {
                let output_val = match gate.op {
                    Op::AND => a_val & b_val,
                    Op::OR => a_val | b_val,
                    Op::XOR => a_val ^ b_val,
                };
                wires.insert(gate.output.clone(), Some(output_val));
                progress = true;
            }
        }

        if !progress {
            break;
        }

        let all_non_null = wires.iter().all(|(key, &val)| !key.starts_with('z') || val.is_some());
        if all_non_null {
            break;
        }
    }

    let mut values: Vec<(String, u32)> = wires
        .iter()
        .filter_map(|(key, &val)| {
            if key.starts_with('z') {
                val.map(|v| (key.clone(), v))
            } else {
                None
            }
        })
        .collect();

    values.sort_by(|a, b| b.0.cmp(&a.0));

    let binary_values: Vec<String> = values.iter().map(|(_, val)| format!("{:b}", val)).collect();
    let binary_string = binary_values.join("");
    let result = u64::from_str_radix(&binary_string, 2).unwrap();

    result
}

#[derive(Debug,Eq, PartialEq)]
enum Op {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct Gate {
    a: String,
    op: Op,
    b: String,
    output: String,
}

fn is_direct(gate: &Gate) -> bool {
    gate.a.starts_with('x') || gate.b.starts_with('x')
}

fn is_output(gate: &Gate) -> bool {
    gate.output.starts_with('z')
}

fn is_gate(gate: &Gate, op: &Op) -> bool {
    &gate.op == op
}

fn has_output(gate: &Gate, output: &str) -> bool {
    gate.output == output
}

fn has_input(gate: &Gate, input: &str) -> bool {
    gate.a == input || gate.b == input
}

pub fn part2(input: &str) -> String {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    let wires_raw: Vec<&str> = parts[0].lines().collect();
    let gates_raw: Vec<&str> = parts[1].lines().collect();

    let mut wires: HashMap<String, Option<u32>> = HashMap::new();
    for line in &wires_raw {
        let parts: Vec<&str> = line.split(": ").collect();
        wires.insert(parts[0].to_string(), Some(parts[1].parse().unwrap()));
    }

    let input_bit_count = wires_raw.len() / 2;

    let mut gates: Vec<Gate> = Vec::new();
    for line in gates_raw {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let inputs: Vec<&str> = parts[0].split_whitespace().collect();
        let gate = Gate {
            a: inputs[0].to_string(),
            op: match inputs[1] {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => panic!("Unknown operation"),
            },
            b: inputs[2].to_string(),
            output: parts[1].to_string(),
        };
        gates.push(gate);

        if !wires.contains_key(inputs[0]) {
            wires.insert(inputs[0].to_string(), None);
        }
        if !wires.contains_key(inputs[2]) {
            wires.insert(inputs[2].to_string(), None);
        }
        if !wires.contains_key(parts[1]) {
            wires.insert(parts[1].to_string(), None);
        }
    }

    let mut flags = HashSet::new();

    let fagate0s: Vec<&Gate> = gates.iter().filter(|g| is_direct(g) && is_gate(g, &Op::XOR)).collect();
    for gate in &fagate0s {
        let is_first = gate.a == "x00" || gate.b == "x00";
        if is_first {
            if gate.output != "z00" {
                flags.insert(gate.output.clone());
            }
            continue;
        }

        if is_output(gate) {
            flags.insert(gate.output.clone());
        }
    }

    let fagate3s: Vec<&Gate> = gates.iter().filter(|g| is_gate(g, &Op::XOR) && !is_direct(g)).collect();
    for gate in &fagate3s {
        if !is_output(gate) {
            flags.insert(gate.output.clone());
        }
    }

    let output_gates: Vec<&Gate> = gates.iter().filter(|g| is_output(g)).collect();
    for gate in &output_gates {
        let is_last = gate.output == format!("z{:02}", input_bit_count);
        if is_last {
            if !matches!(gate.op, Op::OR) {
                flags.insert(gate.output.clone());
            }
            continue;
        } else if !matches!(gate.op, Op::XOR) {
            flags.insert(gate.output.clone());
        }
    }

    let mut check_next = Vec::new();
    for gate in &fagate0s {
        if flags.contains(&gate.output) || gate.output == "z00" {
            continue;
        }

        let matches: Vec<_> = fagate3s.iter().filter(|g| has_input(g, &gate.output)).collect();
        if matches.is_empty() {
            check_next.push(gate);
            flags.insert(gate.output.clone());
        }
    }

    for gate in check_next {
        let intended_result = format!("z{}", &gate.a[1..]);
        let matches:Vec<_> = fagate3s.iter().filter(|&g| has_output(g, &intended_result).clone()).collect();

        if matches.len() != 1 {
            panic!("Critical Error! Is your input correct?");
        }

        let match_gate = &matches[0];
        let to_check = vec![&match_gate.a, &match_gate.b];

        let or_matches: Vec<&Gate> = gates.iter().filter(|g| is_gate(g, &Op::OR) && to_check.contains(&&g.output)).collect();

        if or_matches.len() != 1 {
            panic!("Critical Error! This solver isn't complex enough to solve this");
        }

        let or_match_output = &or_matches[0].output;
        let correct_output = to_check.iter().find(|&&output| output != or_match_output).unwrap();
        flags.insert(correct_output.clone().clone());
    }

    if flags.len() != 8 {
        panic!("Critical Error! This solver isn't complex enough to solve this");
    }

    let mut flags_arr: Vec<String> = flags.into_iter().collect();
    flags_arr.sort();
    flags_arr.join(",")
}

pub fn part3() -> String {
    let input = fs::read_to_string("/Users/johnkryspin/Documents/projects/advent-of-code-rust/input/year2024/day24.txt").unwrap();
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    let wires_raw: Vec<&str> = parts[0].lines().collect();
    let gates_raw: Vec<&str> = parts[1].lines().collect();

    let mut wires: HashMap<String, Option<u32>> = HashMap::new();
    for line in &wires_raw {
        let parts: Vec<&str> = line.split(": ").collect();
        wires.insert(parts[0].to_string(), Some(parts[1].parse().unwrap()));
    }

    let mut gates: Vec<Gate> = Vec::new();
    for line in gates_raw {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let inputs: Vec<&str> = parts[0].split_whitespace().collect();
        let gate = Gate {
            a: inputs[0].to_string(),
            op: match inputs[1] {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => panic!("Unknown operation"),
            },
            b: inputs[2].to_string(),
            output: parts[1].to_string(),
        };
        gates.push(gate);

        if !wires.contains_key(inputs[0]) {
            wires.insert(inputs[0].to_string(), None);
        }
        if !wires.contains_key(inputs[2]) {
            wires.insert(inputs[2].to_string(), None);
        }
        if !wires.contains_key(parts[1]) {
            wires.insert(parts[1].to_string(), None);
        }
    }

    // Create a directed graph
    let mut graph = DiGraph::new();
    let mut node_indices = HashMap::new();

    // Add nodes and edges to the graph
    for gate in &gates {
        let a_index = *node_indices.entry(&gate.a).or_insert_with(|| graph.add_node(gate.a.clone()));
        let b_index = *node_indices.entry(&gate.b).or_insert_with(|| graph.add_node(gate.b.clone()));
        let output_index = *node_indices.entry(&gate.output).or_insert_with(|| graph.add_node(gate.output.clone()));

        graph.add_edge(a_index, output_index, format!("{:?}", gate.op));
        graph.add_edge(b_index, output_index, format!("{:?}", gate.op));
    }

    // Print the graph in DOT format
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    "Graph printed".to_string()
}
