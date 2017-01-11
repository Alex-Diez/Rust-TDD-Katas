use std::collections::HashMap;

pub struct UnidirecredGraph {
    vertices: HashMap<isize, Vec<isize>>,
    edges: usize
}

impl Default for UnidirecredGraph {

    fn default() -> UnidirecredGraph {
        UnidirecredGraph {
            vertices: HashMap::new(),
            edges: 0
        }
    }
}

impl UnidirecredGraph {

    pub fn edges(&self) -> usize {
        self.edges
    }

    pub fn vertices(&self) -> usize {
        self.vertices.len()
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

    pub fn new(graph: &UnidirecredGraph, src: isize) -> DepthFirstSearch {
        let mut dfs = DepthFirstSearch {
            marked: HashMap::default()
        };
        if graph.vertices() > 0 {
            dfs.search(graph, src);
        }
        dfs
    }

    fn search(&mut self, graph: &UnidirecredGraph, v: isize) {
        self.marked.insert(v, true);
        if let Some(adj) = graph.adjacent_to(v) {
            for w in adj {
                if !(*(self.marked.entry(*w).or_insert(false))) {
                    self.search(graph, *w);
                }
            }
        }
    }

    pub fn has_path(&self, v: isize) -> bool {
        self.marked.contains_key(&v)
    }
}
