use petgraph::dot::{Config, Dot};
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use text_io::scan;
type InputType = Graph<String, (), Undirected>;

fn save_graph(g: &InputType) {
    let mut w = File::create("./graph.dot").unwrap();
    writeln!(&mut w, "{:?}", Dot::with_config(g, &[Config::EdgeNoLabel])).unwrap();
}

fn get_input(name: &str) -> InputType {
    let buf = fs::read_to_string(name).unwrap();
    let lines = buf.trim_end().split("\n");
    let mut graph = Graph::new_undirected();
    let mut mapping = HashMap::new();

    for l in lines {
        let start: String;
        let end: String;
        let start_idx;
        let end_idx;
        scan!(l.bytes() => "{}-{}", start, end);
        if let Some(i) = mapping.get(&start) {
            start_idx = *i;
        } else {
            start_idx = graph.add_node(start.clone());
            mapping.insert(start, start_idx);
        }

        if let Some(i) = mapping.get(&end) {
            end_idx = *i;
        } else {
            end_idx = graph.add_node(end.clone());
            mapping.insert(end, end_idx);
        }
        graph.add_edge(start_idx, end_idx, ());
    }
    save_graph(&graph);
    graph
}

fn solve1(g: &InputType) -> usize {
    let start = g.node_indices().find(|i| g[*i] == "start").unwrap();
    let end = g.node_indices().find(|i| g[*i] == "end").unwrap();

    let mut growing = Vec::new();
    let mut ready = Vec::new();

    growing.push(vec![start]);

    while growing.len() > 0 {
        let p = growing.pop().unwrap();
        let last = p.last().unwrap();
        for n in g.neighbors(*last) {
            if g[n].chars().all(|c| c.is_ascii_lowercase()) && p.contains(&n) {
                continue;
            }
            let mut longer = p.clone();
            longer.push(n);
            if n != end {
                growing.push(longer);
            } else {
                ready.push(longer);
            }
        }
    }

    ready.len()
}

fn conatins_double_visit(p: &Vec<petgraph::prelude::NodeIndex>, g: &InputType) -> bool {
    let mut visited = HashSet::new();
    for n in p {
        if g[*n].chars().all(|c| c.is_ascii_lowercase()) {
            if !visited.insert(n) {
                return true;
            }
        }
    }
    false
}

fn solve2(g: &InputType) -> usize {
    let start = g.node_indices().find(|i| g[*i] == "start").unwrap();
    let end = g.node_indices().find(|i| g[*i] == "end").unwrap();

    let mut growing = Vec::new();
    let mut ready = Vec::new();

    growing.push(vec![start]);

    while growing.len() > 0 {
        let p = growing.pop().unwrap();
        let last = p.last().unwrap();
        for n in g.neighbors(*last) {
            if g[n].chars().all(|c| c.is_ascii_lowercase()) && p.contains(&n) {
                if n == start || conatins_double_visit(&p, g) {
                    continue;
                }
            }
            let mut longer = p.clone();
            longer.push(n);
            if n != end {
                growing.push(longer);
            } else {
                ready.push(longer);
            }
        }
    }

    ready.len()
}

fn main() {
    let input = get_input("input.txt");
    println!("1: {}", solve1(&input));
    println!("2: {}", solve2(&input));
}
