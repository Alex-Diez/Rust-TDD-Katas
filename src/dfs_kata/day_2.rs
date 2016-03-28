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

pub struct DepthFirstSearch {
    marked: HashMap<isize, bool>
}

impl DepthFirstSearch {

    pub fn new(graph: &UnidirectedGraph, src: isize) -> DepthFirstSearch {
        let mut dfs = DepthFirstSearch {
            marked: HashMap::with_capacity(graph.vertices())
        };
        if graph.vertices() > 0 {
            dfs.search(graph, src);
        }
        dfs
    }

    fn search(&mut self, graph: &UnidirectedGraph, src: isize) {
        self.marked.insert(src, true);
        if let Some(v) = graph.adjacent_to(src) {
            for w in v {
                if !(*(self.marked.entry(*w).or_insert(false))) {
                    self.search(graph, *w);
                }
            }
        }
    }

    pub fn has_path(&mut self, v: isize) -> bool {
        *(self.marked.entry(v).or_insert(false))
    }
}
