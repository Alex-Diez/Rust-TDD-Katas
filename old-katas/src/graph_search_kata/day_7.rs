use std::collections::{HashMap, HashSet, VecDeque};

pub struct UnidirectedGraph {
    vertices: HashMap<isize, Vec<isize>>,
    edges: usize
}

impl Default for UnidirectedGraph {

    fn default() -> UnidirectedGraph {
        UnidirectedGraph {
            vertices: HashMap::default(),
            edges: 0
        }
    }
}

impl UnidirectedGraph {

    pub fn vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn edges(&self) -> usize {
        self.edges
    }

    pub fn add_edge(&mut self, v: isize, w: isize) {
        self.edges += 1;
        self.vertices.entry(v).or_insert_with(Vec::new).push(w);
        self.vertices.entry(w).or_insert_with(Vec::new).push(v);
    }

    pub fn adjacent_to(&self, v: isize) -> Option<&Vec<isize>> {
        self.vertices.get(&v)
    }
}

#[derive(Debug)]
pub struct DepthFirstSearch {
    marked: HashSet<isize>
}

impl DepthFirstSearch {

    pub fn new(graph: &UnidirectedGraph, src: isize) -> Option<DepthFirstSearch> {
        if graph.vertices() > 1 {
            let mut dfs = DepthFirstSearch {
                marked: HashSet::with_capacity(graph.vertices())
            };
            dfs.search(graph, src);
            Some(dfs)
        }
        else {
            None
        }
    }

    fn search(&mut self, graph: &UnidirectedGraph, v: isize) {
        self.marked.insert(v);
        if let Some(adj) = graph.adjacent_to(v) {
            for w in adj {
                if !self.marked.contains(w) {
                    self.search(graph, *w);
                }
            }
        }
    }

    pub fn has_path(&self, v: isize) -> bool {
        self.marked.contains(&v)
    }
}

#[derive(Debug)]
pub struct BreadthFirstSearch {
    marked: HashSet<isize>
}

impl BreadthFirstSearch {

    pub fn new(graph: &UnidirectedGraph, src: isize) -> Option<BreadthFirstSearch> {
        if graph.vertices() > 1 {
            let mut bfs = BreadthFirstSearch {
                marked: HashSet::with_capacity(graph.vertices())
            };
            bfs.search(graph, src);
            Some(bfs)
        }
        else {
            None
        }
    }

    fn search(&mut self, graph: &UnidirectedGraph, s: isize) {
        self.marked.insert(s);
        let mut queue = VecDeque::with_capacity(graph.vertices());
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            if let Some(adj) = graph.adjacent_to(v) {
                for w in adj {
                    if !self.marked.contains(w) {
                        self.marked.insert(*w);
                        queue.push_back(*w);
                    }
                }
            }
        }
    }

    pub fn has_path(&self, v: isize) -> bool {
        self.marked.contains(&v)
    }
}
