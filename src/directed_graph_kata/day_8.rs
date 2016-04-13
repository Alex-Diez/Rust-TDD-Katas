use std::collections::{HashMap, HashSet};

pub struct DirectedGraph {
    edges: HashMap<isize, Vec<isize>>
}

impl Default for DirectedGraph {

    fn default() -> DirectedGraph {
        DirectedGraph {
            edges: HashMap::default()
        }
    }
}

impl DirectedGraph {

    pub fn vertices(&self) -> usize {
        self.edges.len()
    }


    pub fn edges(&self) -> usize {
        self.edges.values().fold(0, |acc, v| acc + v.len())
    }

    pub fn add_edge(&mut self, v: isize, w: isize) {
        self.edges.entry(v).or_insert_with(Vec::new).push(w);
        self.edges.entry(w).or_insert_with(Vec::new);
    }

    pub fn adjacent_to(&self, v :isize) -> Option<&Vec<isize>> {
        self.edges.get(&v)
    }
}

#[derive(Debug)]
pub struct DirectedCycle {
    marked: HashSet<isize>,
    on_stack: HashSet<isize>,
    has_cycle: bool
}

impl DirectedCycle {

    pub fn new(graph: &DirectedGraph) -> Result<DirectedCycle, ()> {
        if graph.vertices() > 0 {
            let mut dc = DirectedCycle {
                marked: HashSet::default(),
                on_stack: HashSet::default(),
                has_cycle: false
            };
            for v in graph.edges.keys() {
                if !dc.marked.contains(v) {
                    dc.find_cycle(graph, *v);
                }
            }
            Ok(dc)
        }
        else {
            Err(())
        }
    }

    fn find_cycle(&mut self, graph: &DirectedGraph, v: isize) {
        self.marked.insert(v);
        self.on_stack.insert(v);
        if let Some(adj) = graph.adjacent_to(v) {
            for w in adj {
                if self.has_cycle() { return; }
                else if !self.marked.contains(w) {
                    self.find_cycle(graph, *w);
                }
                else if self.on_stack.contains(w) {
                    self.has_cycle = true;
                }
            }
        }
        self.on_stack.remove(&v);
    }

    pub fn has_cycle(&self) -> bool {
        self.has_cycle
    }
}
