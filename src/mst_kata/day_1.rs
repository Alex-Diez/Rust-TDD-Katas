use std::collections::{HashMap, HashSet, BinaryHeap, VecDeque};
use std::cmp::Ordering;

#[derive(Eq, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct WeightedEdge {
    v: usize,
    w: usize,
    weight: usize
}

impl WeightedEdge {

    pub fn new(v: usize, w: usize, weight: usize) -> WeightedEdge {
        WeightedEdge {
            v: v,
            w: w,
            weight: weight
        }
    }

    pub fn weight(&self) -> usize {
        self.weight
    }

    pub fn either(&self) -> usize {
        self.v
    }

    pub fn other(&self, v: usize) -> usize {
        if v == self.v {
            self.w
        }
        else {
            self.v
        }
    }
}

impl Ord for WeightedEdge {

    fn cmp(&self, other: &WeightedEdge) -> Ordering {
        if self.weight < other.weight { Ordering::Greater }
        else if self.weight > other.weight { Ordering::Less }
        else { Ordering::Equal }
    }
}

pub struct EdgeWeightedGraph {
    edges: HashMap<usize, Vec<WeightedEdge>>
}

impl Default for EdgeWeightedGraph {

    fn default() -> EdgeWeightedGraph {
        EdgeWeightedGraph {
            edges: HashMap::default()
        }
    }
}

impl EdgeWeightedGraph {

    pub fn vertices(&self) -> usize {
        self.edges.len()
    }

    pub fn edges(&self) -> usize {
        self.edges.values().fold(0, |acc, v| acc + v.len()) / 2
    }

    pub fn add_edge(&mut self, edge: WeightedEdge) {
        let v = edge.either();
        self.edges.entry(v).or_insert_with(Vec::new).push(edge);
        self.edges.entry(edge.other(v)).or_insert_with(Vec::new).push(edge);
    }

    pub fn adjacent_to(&self, v: usize) -> Option<&Vec<WeightedEdge>> {
        self.edges.get(&v)
    }
}

#[derive(Debug)]
pub struct LazyMst {
    marked: HashSet<usize>,
    pq: BinaryHeap<WeightedEdge>,
    weight: usize
}

impl LazyMst {

    pub fn new(graph: &EdgeWeightedGraph) -> Result<LazyMst, ()> {
        if graph.vertices() > 0 {
            let mut lazyMst = LazyMst {
                marked: HashSet::default(),
                pq: BinaryHeap::default(),
                weight: 0
            };
            let mut mst = VecDeque::default();
            lazyMst.visit(graph, *graph.edges.keys().next().unwrap());
            while let Some(edge) = lazyMst.pq.pop() {
                let v = edge.either();
                let w = edge.other(v);
                if !lazyMst.marked.contains(&v)
                        || !lazyMst.marked.contains(&w) {
                    mst.push_back(edge);
                    if !lazyMst.marked.contains(&v) {
                        lazyMst.visit(graph, v);
                    }
                    if !lazyMst.marked.contains(&w) {
                        lazyMst.visit(graph, w);
                    }
                }
            }
            lazyMst.weight = mst.iter().fold(0, |acc, e| acc + e.weight);
            Ok(lazyMst)
        }
        else {
            Err(())
        }
    }

    fn visit(&mut self, graph: &EdgeWeightedGraph, v: usize) {
        self.marked.insert(v);
        if let Some(adj) = graph.adjacent_to(v) {
            for e in adj {
                if !self.marked.contains(&e.other(v)) {
                    self.pq.push(*e);
                }
            }
        }
    }

    pub fn weight(&self) -> usize {
        self.weight
    }
}
