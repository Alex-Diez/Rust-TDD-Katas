use std::collections::HashMap;

pub struct UnidirectedGraph {
    vertices: HashMap<isize, Vec<isize>>,
    edges: usize
}

impl Default for UnidirectedGraph {

    fn default() -> UnidirectedGraph {
        UnidirectedGraph {
            vertices: HashMap::new(),
            edges: 0
        }
    }
}

impl UnidirectedGraph {

    pub fn add_edge(&mut self, v: isize, w: isize) {
        self.vertices.entry(v).or_insert_with(Vec::new).push(w);
        self.vertices.entry(w).or_insert_with(Vec::new).push(v);
        self.edges += 1;
    }

    pub fn edges(&self) -> usize {
        self.edges
    }

    pub fn vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn adjacent_to(&self, v: isize) -> &Vec<isize> {
        &(self.vertices[&v])
    }
}

pub struct DepthFirstSearch {
    marked: HashMap<isize, bool>,
    edge_to: HashMap<isize, isize>
}

impl DepthFirstSearch {

    pub fn new(graph: &UnidirectedGraph, source: isize) -> DepthFirstSearch {
        let len = graph.vertices();
        let mut dfs = DepthFirstSearch {
            marked: HashMap::with_capacity(len),
            edge_to: HashMap::with_capacity(len)
        };
        if len > 1 {
            dfs.dfs(graph, source);
        }
        dfs
    }

    fn dfs(&mut self, graph: &UnidirectedGraph, v: isize) {
        self.marked.insert(v, true);
        for w in graph.adjacent_to(v) {
            if !(*(self.marked.entry(*w).or_insert(false))) {
                self.edge_to.entry(*w).or_insert(v);
                self.dfs(graph, *w);
            }
        }
    }

    pub fn has_path(&mut self, v: isize) -> bool {
        *(self.marked.entry(v).or_insert(false))
    }
}
