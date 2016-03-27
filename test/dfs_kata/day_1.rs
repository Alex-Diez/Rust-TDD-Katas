pub use tdd_kata::dfs_kata::day_1::{UnidirectedGraph, DepthFirstSearch};

pub use expectest::prelude::{be_equal_to, be_true, be_false};

describe! dfs {

    describe! graph {

        before_each {
            let mut graph = UnidirectedGraph::default();

            graph.add_edge(1, 2);
        }

        it "should add edge to graph" {
            expect!(graph.edges()).to(be_equal_to(1));
        }

        it "should add number of edges to graph" {
            graph.add_edge(2, 3);
            graph.add_edge(3, 4);
            expect!(graph.edges()).to(be_equal_to(3));
        }

        it "should contain all added vertices" {
            graph.add_edge(2, 3);
            graph.add_edge(3, 4);
            expect!(graph.vertices()).to(be_equal_to(4));
        }

        it "should be adjacent to each other" {
            let adj_to_one = graph.adjacent_to(1);
            expect!(adj_to_one).to(be_equal_to(&vec![2]));

            let adj_to_two = graph.adjacent_to(2);
            expect!(adj_to_two).to(be_equal_to(&vec![1]));
        }

    }

    describe! search {

        it "should create depth first search" {
            let graph = UnidirectedGraph::default();
            DepthFirstSearch::new(&graph, 1);
        }

        it "should have path between adjacent vertices" {
            let mut graph = UnidirectedGraph::default();
            graph.add_edge(1, 2);

            let mut search = DepthFirstSearch::new(&graph, 1);

            expect!(search.has_path(2)).to(be_true());
        }

        it "should not have path between not adjacent vertices" {
            let mut graph = UnidirectedGraph::default();
            graph.add_edge(1, 2);
            graph.add_edge(3, 4);

            let mut search = DepthFirstSearch::new(&graph, 1);

            expect!(search.has_path(4)).to(be_false());
        }
    }
}
