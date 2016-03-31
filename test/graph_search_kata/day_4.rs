pub use tdd_kata::graph_search_kata::day_4::{UnidirecredGraph, DepthFirstSearch};

pub use expectest::prelude::{be_equal_to, be_some, be_true, be_false};

describe! dfs {

    describe! graph {

        before_each {
            let mut graph = UnidirecredGraph::default();
        }

        it "should create a new empty graph" {
            expect!(graph.edges()).to(be_equal_to(0));
            expect!(graph.vertices()).to(be_equal_to(0));
        }

        it "should add edge to a graph" {
            graph.add_edge(1, 2);

            expect!(graph.edges()).to(be_equal_to(1));
            expect!(graph.vertices()).to(be_equal_to(2));
        }

        it "should add edges to a graph" {
            graph.add_edge(1, 2);
            graph.add_edge(1, 3);
            graph.add_edge(2, 4);

            expect!(graph.vertices()).to(be_equal_to(4));
            expect!(graph.edges()).to(be_equal_to(3));
        }

        it "should be adjacent to each other" {
            graph.add_edge(1, 2);

            expect!(graph.adjacent_to(1)).to(be_some().value(&vec![2]));
            expect!(graph.adjacent_to(2)).to(be_some().value(&vec![1]));
        }
    }

    describe! search {

        it "should create a depth first search" {
            let graph = UnidirecredGraph::default();
            DepthFirstSearch::new(&graph, 1);
        }

        it "should have path to transetive vertices" {
            let mut graph = UnidirecredGraph::default();
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            let search = DepthFirstSearch::new(&graph, 1);

            expect!(search.has_path(2)).to(be_true());
        }

        it "shold not have path to not connected vertices" {
            let mut graph = UnidirecredGraph::default();
            graph.add_edge(1, 2);
            graph.add_edge(3, 4);
            let search = DepthFirstSearch::new(&graph, 1);

            expect!(search.has_path(3)).to(be_false());
        }
    }

}
