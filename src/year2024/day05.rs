use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn part1(input: &str) -> u32 {
    let printer = Printer::new(input);
    let (ordered_pages, _) = printer.get_ordered_pages();
    let middle_pages_sum: u32 = ordered_pages.iter().map(|p| p.get_middle_page()).sum();
    middle_pages_sum
}

pub fn part2(input: &str) -> u32 {
    let printer = Printer::new(input);
    let (_, unordered_pages) = printer.get_ordered_pages();
    let mut sum = 0;

    for page_producer in unordered_pages {
        let rules = printer.filter_rules(&page_producer.pages);
        let graph = build_graph(&rules);
        let sorter = Sorter::new();
        let sorted_pages = sorter.topological_sort(&graph);
        sum += sorted_pages[sorted_pages.len() / 2];
    }

    sum
}

fn build_graph(rules: &[&PageOrderingRule]) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for rule in rules {
        graph
            .entry(rule.first)
            .or_insert_with(Vec::new)
            .push(rule.second);
    }
    graph
}

#[derive(Debug)]
struct Printer {
    page_ordering_rules: Vec<PageOrderingRule>,
    pages_to_produce: Vec<PageProducer>,
}

impl Printer {
    fn new(input: &str) -> Self {
        let (rules, pages) = input.split_once("\n\n").unwrap();
        let page_ordering_rules = rules.lines().map(PageOrderingRule::from).collect();
        let pages_to_produce = pages.lines().map(PageProducer::from).collect();
        Self {
            page_ordering_rules,
            pages_to_produce,
        }
    }

    fn get_ordered_pages(&self) -> (Vec<&PageProducer>, Vec<&PageProducer>) {
        self.pages_to_produce
            .iter()
            .partition(|&p| p.is_ordered(&self.page_ordering_rules))
    }

    fn filter_rules(&self, pages: &[u32]) -> Vec<&PageOrderingRule> {
        self.page_ordering_rules
            .iter()
            .filter(|rule| pages.contains(&rule.first) && pages.contains(&rule.second))
            .collect()
    }
}

#[derive(Debug)]
struct PageOrderingRule {
    first: u32,
    second: u32,
}

impl From<&str> for PageOrderingRule {
    fn from(line: &str) -> Self {
        let (first, second) = line.split_once("|").unwrap();
        Self {
            first: first.parse().unwrap(),
            second: second.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct PageProducer {
    pages: Vec<u32>,
}

impl From<&str> for PageProducer {
    fn from(line: &str) -> Self {
        Self {
            pages: line.split(",").map(|p| p.parse().unwrap()).collect(),
        }
    }
}

impl PageProducer {
    fn get_middle_page(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }

    fn is_ordered(&self, rules: &[PageOrderingRule]) -> bool {
        rules.iter().all(|rule| {
            let first_pos = self.pages.iter().position(|&p| p == rule.first);
            let second_pos = self.pages.iter().position(|&p| p == rule.second);
            match (first_pos, second_pos) {
                (Some(f), Some(s)) => f < s,
                _ => true,
            }
        })
    }
}

pub struct Sorter<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Eq + Hash + Copy> Sorter<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn topological_sort(&self, graph: &HashMap<T, Vec<T>>) -> Vec<T> {
        let mut sorted = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_mark = HashSet::new();

        for &node in graph.keys() {
            if !visited.contains(&node) {
                self.visit(node, graph, &mut visited, &mut temp_mark, &mut sorted);
            }
        }

        sorted.reverse();
        sorted
    }

    fn visit(
        &self,
        node: T,
        graph: &HashMap<T, Vec<T>>,
        visited: &mut HashSet<T>,
        temp_mark: &mut HashSet<T>,
        sorted: &mut Vec<T>,
    ) {
        if temp_mark.contains(&node) {
            panic!("Graph has a cycle");
        }

        if !visited.contains(&node) {
            temp_mark.insert(node);
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    self.visit(neighbor, graph, visited, temp_mark, sorted);
                }
            }
            temp_mark.remove(&node);
            visited.insert(node);
            sorted.push(node);
        }
    }
}