use std::collections::{HashMap, HashSet};

pub struct DirectedGraph {
    adjacent: HashMap<isize, Vec<isize>>,
    vertices: HashSet<isize>,
    edges: usize
}

impl Default for DirectedGraph {

    fn default() -> DirectedGraph {
        DirectedGraph {
            adjacent: HashMap::default(),
            vertices: HashSet::default(),
            edges: 0
        }
    }
}

impl DirectedGraph {

    pub fn vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn edges(&self) -> usize {
        self.edges
    }

    pub fn add_edge(&mut self, v: isize, w: isize) {
        self.edges += 1;
        self.adjacent.entry(v).or_insert_with(Vec::new).push(w);
        self.vertices.insert(w);
        self.vertices.insert(v);
    }

    pub fn adjacent_to(&self, v: isize) -> Option<&Vec<isize>> {
        self.adjacent.get(&v)
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
            for v in &graph.vertices {
                if !dc.marked.contains(v) {
                    dc.depth_first_search(graph, *v);
                }
            }
            Ok(dc)
        }
        else {
            Err(())
        }
    }

    fn depth_first_search(&mut self, graph: &DirectedGraph, v: isize) {
        self.on_stack.insert(v);
        self.marked.insert(v);
        if let Some(adj) = graph.adjacent_to(v) {
            for w in adj {
                if self.has_cycle() {
                    return;
                }
                else if !self.marked.contains(w) {
                    self.depth_first_search(graph, *w);
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

#[derive(Debug)]
pub struct TopologySort {
    acyclic: bool
}

impl TopologySort {

    pub fn new(graph: &DirectedGraph) -> Result<TopologySort, ()> {
        let dc = DirectedCycle::new(graph);
        match dc {
            Ok(dc) => Ok(TopologySort { acyclic: !dc.has_cycle() } ),
            Err(_) => Err(()),
        }
    }

    pub fn is_acyclic_graph(&self) -> bool {
        self.acyclic
    }
}
