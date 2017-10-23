use std::collections::HashMap;
use std::collections::VecDeque;

pub struct UnidirecredGraph {
    vertices: HashMap<isize, Vec<isize>>,
    edges: usize
}

impl Default for UnidirecredGraph {

    fn default() -> UnidirecredGraph {
        UnidirecredGraph {
            vertices: HashMap::default(),
            edges: 0
        }
    }
}

impl UnidirecredGraph {

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
    marked: HashMap<isize, bool>
}

impl DepthFirstSearch {

    pub fn new(graph: &UnidirecredGraph, src: isize) -> Option<DepthFirstSearch> {
        let size = graph.vertices();
        if size > 1 {
            let mut dfs = DepthFirstSearch {
                marked: HashMap::with_capacity(size)
            };
            dfs.search(graph, src);
            Some(dfs)
        }
        else {
            None
        }
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

#[derive(Debug)]
pub struct BreadthFirstSearch {
    marked: HashMap<isize, bool>
}

impl BreadthFirstSearch {

    pub fn new(graph: &UnidirecredGraph, src: isize) -> Option<BreadthFirstSearch> {
        let size = graph.vertices();
        if size > 1 {
            let mut bfs = BreadthFirstSearch {
                marked: HashMap::with_capacity(size)
            };
            bfs.search(graph, src);
            Some(bfs)
        }
        else {
            None
        }
    }

    fn search(&mut self, graph: &UnidirecredGraph, s: isize) {
        let mut queue = VecDeque::with_capacity(graph.vertices());
        self.marked.insert(s, true);
        queue.push_front(s);
        while let Some(v) = queue.pop_back() {
            if let Some(adj) = graph.adjacent_to(v) {
                for w in adj {
                    if !(*(self.marked.entry(*w).or_insert(false))) {
                        self.marked.insert(*w, true);
                        queue.push_front(*w);
                    }
                }
            }
        }
    }

    pub fn has_path(&self, v: isize) -> bool {
        self.marked.contains_key(&v)
    }
}
