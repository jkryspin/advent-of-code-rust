use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> i64 {
    let tree = Tree::from_input(input);
    // subtract 1 because the root node is not a direct orbit
    tree.nodes.keys().map(|node| tree.depth("COM",node)).sum::<u32>() as i64
}

pub fn part2(input: &str) -> u32 {
    let tree = Tree::from_input(input);
    let path = tree.distance("YOU", "SAN");
    // subtract 2 because the path includes the start and end node
    println!("{:?}", path);
    path - 2
}

struct Node {
    name: String,
    children: Vec<String>,
}

struct Tree {
    nodes: HashMap<String, Node>,
    root: String,
}

impl Tree {
    fn from_input(input: &str) -> Self {
        let mut nodes = HashMap::new();
        for line in input.lines() {
            let mut parts = line.split(')');
            let parent = parts.next().unwrap();
            let child = parts.next().unwrap();
            nodes.entry(parent.to_string()).or_insert(Node {
                name: parent.to_string(),
                children: vec![],
            });
            nodes.entry(child.to_string()).or_insert(Node {
                name: child.to_string(),
                children: vec![],
            });
            nodes.get_mut(parent).unwrap().children.push(child.to_string());
        }
        Tree {
            nodes,
            root: "COM".to_string(),
        }
    }
    fn depth(&self,source:&str, target: &str) -> u32 {
        let mut stack = vec![(source, 0)];
        while let Some((node, d)) = stack.pop() {
            if node == target {
                return d;
            }
            for child in &self.nodes[node].children {
                stack.push((child, d + 1));
            }
        }
        unreachable!("Node not found");
    }
    fn distance(&self, start: &str, end: &str) -> u32 {
        let mut stack = vec![("COM", vec![])];
        let mut path_start = vec![];
        let mut path_end = vec![];
        while let Some((node, path)) = stack.pop() {
            if node == start {
                path_start = path.clone();
                continue;
            }
            if node == end {
                path_end = path.clone();
                continue;
            }
            for child in &self.nodes[node].children {
                let mut new_path = path.clone();
                new_path.push(node);
                stack.push((child, new_path));
            }
        }
        println!("{:?} {:?}", path_start, path_end);
        // get the last common node
        let root = path_start
            .iter()
            .zip(path_end.iter())
            .take_while(|(a, b)| a == b)
            .last()
            .unwrap()
            .0
            .to_string();

        self.depth(&root, start) + self.depth(&root, end)
    }
}

