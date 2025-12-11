//! # Day 11: [Title]

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    outputs: Vec<usize>, // indices into the nodes vector
}

fn parse_graph(input: &str) -> Vec<Node> {
    let mut name_to_index: HashMap<String, usize> = HashMap::new();
    let mut connections: Vec<(String, Vec<String>)> = Vec::new();

    // First pass: collect all node names and their connections
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            continue;
        }

        let source = parts[0].to_string();
        let targets: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        connections.push((source, targets));
    }

    // Build name_to_index map
    let mut node_names: Vec<String> = Vec::new();
    for (source, targets) in &connections {
        if !name_to_index.contains_key(source) {
            name_to_index.insert(source.clone(), node_names.len());
            node_names.push(source.clone());
        }
        for target in targets {
            if !name_to_index.contains_key(target) {
                name_to_index.insert(target.clone(), node_names.len());
                node_names.push(target.clone());
            }
        }
    }

    // Create nodes with empty outputs
    let mut nodes: Vec<Node> = node_names
        .iter()
        .map(|name| Node {
            name: name.clone(),
            outputs: Vec::new(),
        })
        .collect();

    // Fill in the connections
    for (source, targets) in connections {
        let source_idx = name_to_index[&source];
        for target in targets {
            let target_idx = name_to_index[&target];
            nodes[source_idx].outputs.push(target_idx);
        }
    }

    nodes
}

fn count_paths(
    nodes: &[Node],
    current: usize,
    target: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    // If we've already computed paths from this node, return cached result
    if let Some(&count) = memo.get(&current) {
        return count;
    }

    // Base case: we've reached the target
    if current == target {
        return 1;
    }

    // Recursive case: sum paths from all outputs
    let mut total_paths = 0;
    for &output in &nodes[current].outputs {
        total_paths += count_paths(nodes, output, target, memo);
    }

    // Cache the result
    memo.insert(current, total_paths);
    total_paths
}

fn print_graph(nodes: &[Node]) {
    println!("\n=== Graph Structure ===");
    for (i, node) in nodes.iter().enumerate() {
        if node.outputs.is_empty() {
            println!("{:3} [{}]", i, node.name);
        } else {
            println!("{:3} [{}] ->", i, node.name);
            for (j, &output_idx) in node.outputs.iter().enumerate() {
                let is_last = j == node.outputs.len() - 1;
                let prefix = if is_last { "    └─" } else { "    ├─" };
                println!("{} {} [{}]", prefix, output_idx, nodes[output_idx].name);
            }
        }
    }
    println!("=======================\n");
}

pub fn part1(input: &str) -> usize {
    let nodes = parse_graph(input);

    // Visualize the graph
    print_graph(&nodes);

    // Find "you" and "out" nodes
    let you_idx = nodes.iter().position(|n| n.name == "you").unwrap();
    let out_idx = nodes.iter().position(|n| n.name == "out").unwrap();

    let mut memo = HashMap::new();
    count_paths(&nodes, you_idx, out_idx, &mut memo)
}

fn count_paths_with_required(
    nodes: &[Node],
    current: usize,
    target: usize,
    required1: usize,
    required2: usize,
    visited_req1: bool,
    visited_req2: bool,
    memo: &mut HashMap<(usize, bool, bool), usize>,
) -> usize {
    // Check memoization
    let state = (current, visited_req1, visited_req2);
    if let Some(&count) = memo.get(&state) {
        return count;
    }

    // Update visited status for required nodes
    let new_visited_req1 = visited_req1 || current == required1;
    let new_visited_req2 = visited_req2 || current == required2;

    // Base case: we've reached the target
    if current == target {
        // Only count if we've visited both required nodes
        return if new_visited_req1 && new_visited_req2 { 1 } else { 0 };
    }

    // Recursive case: sum paths from all outputs
    let mut total_paths = 0;
    for &output in &nodes[current].outputs {
        total_paths += count_paths_with_required(
            nodes,
            output,
            target,
            required1,
            required2,
            new_visited_req1,
            new_visited_req2,
            memo,
        );
    }

    // Cache the result
    memo.insert(state, total_paths);
    total_paths
}

pub fn part2(input: &str) -> usize {
    let nodes = parse_graph(input);

    // Find required nodes
    let svr_idx = nodes.iter().position(|n| n.name == "svr").unwrap();
    let out_idx = nodes.iter().position(|n| n.name == "out").unwrap();
    let dac_idx = nodes.iter().position(|n| n.name == "dac").unwrap();
    let fft_idx = nodes.iter().position(|n| n.name == "fft").unwrap();

    let mut memo = HashMap::new();
    count_paths_with_required(&nodes, svr_idx, out_idx, dac_idx, fft_idx, false, false, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE2), 2);
    }
}
