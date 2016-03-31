pub use tdd_kata::graph_search_kata::day_5::{UnidirecredGraph, DepthFirstSearch, BreadthFirstSearch};

pub use expectest::prelude::{be_equal_to, be_some, be_none, be_true, be_false};

describe! search {

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

            expect!(graph.vertices()).to(be_equal_to(2));
            expect!(graph.edges()).to(be_equal_to(1));
        }

        it "should add edges to a graph" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(1, 4);

            expect!(graph.edges()).to(be_equal_to(3));
            expect!(graph.vertices()).to(be_equal_to(4));
        }

        it "should be adjacent to each other" {
            graph.add_edge(1, 2);

            expect!(graph.adjacent_to(1)).to(be_some().value(&vec![2]));
            expect!(graph.adjacent_to(2)).to(be_some().value(&vec![1]));
        }
    }

    describe! depth_first {

        before_each {
            let mut graph = UnidirecredGraph::default();
        }

        it "should create depth first search" {
            graph.add_edge(1, 2);
            expect!(DepthFirstSearch::new(&graph, 1)).to(be_some());
        }

        it "should not create depth first search from empty graph" {
            expect!(DepthFirstSearch::new(&graph, 1)).to(be_none());
        }

        it "should have path to transient vertieces" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);

            let search = DepthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(3)).to(be_true());
        }

        it "should not have path to not adjacent vertices" {
            graph.add_edge(1, 2);
            graph.add_edge(3, 4);

            let search = DepthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(4)).to(be_false());
        }
    }

    describe! breadth_first {

        before_each {
            let mut graph = UnidirecredGraph::default();
        }

        it "should create breadth first search" {
            graph.add_edge(1, 2);
            expect!(BreadthFirstSearch::new(&graph, 1)).to(be_some());
        }

        it "should not create breadth first search from empty graph" {
            expect!(BreadthFirstSearch::new(&graph, 1)).to(be_none());
        }

        it "should have path to transient vertieces" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);

            let search = BreadthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(3)).to(be_true());
        }

       it "should not have path to not adjacent vertices" {
            graph.add_edge(1, 2);
            graph.add_edge(3, 4);

            let search = BreadthFirstSearch::new(&graph, 1).unwrap();

            expect!(search.has_path(4)).to(be_false());
        }
    }
}
