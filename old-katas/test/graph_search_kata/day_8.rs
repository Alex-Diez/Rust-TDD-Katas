pub use tdd_kata::graph_search_kata::day_8::{UnidirectGraph, DepthFirstSearch, BreadthFirstSearch};

pub use expectest::prelude::{be_equal_to, be_some, be_none, be_true, be_false};

describe! graph_search {

    describe! graph {

        it "should create a new empty graph" {
            let graph = UnidirectGraph::default();

            expect!(graph.vertices()).to(be_equal_to(0));
            expect!(graph.edges()).to(be_equal_to(0));
        }

        it "should add an edge to a graph" {
            let mut graph = UnidirectGraph::default();

            graph.add_edge(1, 2);

            expect!(graph.edges()).to(be_equal_to(1));
            expect!(graph.vertices()).to(be_equal_to(2));
        }

        it "should add edges to a graph" {
            let mut graph = UnidirectGraph::default();

            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(1, 4);

            expect!(graph.edges()).to(be_equal_to(3));
            expect!(graph.vertices()).to(be_equal_to(4));
        }

        it "should be adjacent to each other" {
            let mut graph = UnidirectGraph::default();

            graph.add_edge(1, 2);

            expect!(graph.adjacent_to(1)).to(be_some().value(&vec![2]));
            expect!(graph.adjacent_to(2)).to(be_some().value(&vec![1]));
        }
    }

    describe! dfs {

        it "should create a depth first search" {
            let mut graph = UnidirectGraph::default();

            graph.add_edge(1, 2);

            expect!(DepthFirstSearch::new(&graph, 1)).to(be_some());
        }

        it "should not create a depth first search from an empty graph" {
            let graph = UnidirectGraph::default();

            expect!(DepthFirstSearch::new(&graph, 1)).to(be_none());
        }

        it "should have path to transient vertices" {
            let mut graph = UnidirectGraph::default();

            graph.add_edge(1, 2);
            graph.add_edge(2, 3);

            let search = DepthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(3)).to(be_true());
        }

        it "should not have path to not connected vertices" {
            let mut graph = UnidirectGraph::default();

            graph.add_edge(1, 2);
            graph.add_edge(3, 4);

            let search = DepthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(3)).to(be_false());
        }
    }

    describe! bfs {

        it "should create a breadth first search" {
            let mut graph = UnidirectGraph::default();

            graph.add_edge(1, 2);

            expect!(BreadthFirstSearch::new(&graph, 1)).to(be_some());
        }

        it "should not create a breadth first seach from an empty graph" {
            let graph = UnidirectGraph::default();

            expect!(BreadthFirstSearch::new(&graph, 1)).to(be_none());
        }

        it "should have path to transient vertices" {
            let mut graph = UnidirectGraph::default();

            graph.add_edge(1, 2);
            graph.add_edge(2, 3);

            let search = BreadthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(3)).to(be_true());
        }

        it "should not have path to not connected vertices" {
            let mut graph = UnidirectGraph::default();

            graph.add_edge(1, 2);
            graph.add_edge(3, 4);

            let search = BreadthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(3)).to(be_false());
        }
    }
}
