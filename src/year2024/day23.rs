use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input: &str) -> i64 {
    let graph = parse(input);
    let mut cycles = HashSet::new();

    for node in graph.nodes.values() {
        let mut queue = VecDeque::new();
        queue.push_back((node.id.clone(), vec![node.id.clone()]));

        while let Some((current, path)) = queue.pop_front() {
            if path.len() == 4 {
                if path[0] == current {
                    let mut cycle = path.clone();
                    cycle.sort();
                    cycles.insert(cycle);
                }
                continue;
            }

            if let Some(next_node) = graph.nodes.get(&current) {
                for edge in &next_node.edges {
                        let mut new_path = path.clone();
                        new_path.push(edge.clone());
                        queue.push_back((edge.clone(), new_path));
                }
            }
        }
    }
    // get distinct ids
    let distinct:Vec<Vec<String>> = cycles.iter().map(|x| {
        let mut x = x.clone();
        x.sort();
        x.dedup();
        x
    }).collect();
    let distinct:HashSet<Vec<String>> = distinct.iter().cloned().collect();
    let name_starts_with_t = distinct.iter().filter(|x| x.iter().any(|s|s.starts_with("t"))).count();

    name_starts_with_t as i64
}

pub fn part2(input: &str) -> String {
    let graph = parse(input);
    let mut maximal_clique = Vec::new();
    let mut current_clique = HashSet::new();
    let mut potential_clique: HashSet<String> = graph.nodes.keys().cloned().collect();
    let mut excluded_nodes = HashSet::new();

    bron_kerbosch(&graph, &mut current_clique, &mut potential_clique, &mut excluded_nodes, &mut maximal_clique);

    maximal_clique.sort();
    maximal_clique.join(",")
}

fn bron_kerbosch(graph: &Graph, current_clique: &mut HashSet<String>, potential_clique: &mut HashSet<String>, excluded_nodes: &mut HashSet<String>, maximal_clique: &mut Vec<String>) {
    if potential_clique.is_empty() && excluded_nodes.is_empty() {
        if current_clique.len() > maximal_clique.len() {
            *maximal_clique = current_clique.clone().into_iter().collect();
        }
        return;
    }

    let potential_clique_clone = potential_clique.clone();
    for node in potential_clique_clone {
        let mut new_clique = current_clique.clone();
        new_clique.insert(node.clone());

        let neighbors: HashSet<String> = graph.nodes[&node].edges.clone();
        let mut new_potential_clique: HashSet<String> = potential_clique.intersection(&neighbors).cloned().collect();
        let mut new_excluded_nodes: HashSet<String> = excluded_nodes.intersection(&neighbors).cloned().collect();

        bron_kerbosch(graph, &mut new_clique, &mut new_potential_clique, &mut new_excluded_nodes, maximal_clique);

        potential_clique.remove(&node);
        excluded_nodes.insert(node);
    }
}
fn parse(input: &str) -> Graph {
    let mut graph = Graph { nodes: HashMap::new() };
    for line in input.lines() {
        if let Some((left, right)) = line.split_once("-") {
            graph.add_node(left.to_string(), right.to_string());
            graph.add_node(right.to_string(), left.to_string());
        }
    }
    graph
}

struct Graph {
    nodes: HashMap<String, Node>,
}
impl Graph {
    fn add_node(&mut self, id: String, edge: String) {
        if self.nodes.contains_key(&id) {
            // add edge
            self.nodes.get_mut(&id).unwrap().edges.insert(edge);
        }
        else{
            let mut edges = HashSet::new();
            edges.insert(edge);
            self.nodes.insert(id.clone(), Node{edges,id:id.clone()});
        }
    }
}

struct Node {
    edges: HashSet<String>,
    id: String,
}