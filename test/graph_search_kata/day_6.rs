pub use tdd_kata::graph_search_kata::day_6::{UnidirectedGraph, DepthFirstSearch, BreadthFirstSearch};

pub use expectest::prelude::{be_equal_to, be_some, be_none, be_true, be_false};


describe! search {
    describe! graph {

        before_each {
            let mut graph = UnidirectedGraph::default();
        }

        it "should create a new graph" {
            expect!(graph.edges()).to(be_equal_to(0));
            expect!(graph.vertices()).to(be_equal_to(0));
        }

        it "should add an edge to a graph" {
            graph.add_edge(1, 2);

            expect!(graph.edges()).to(be_equal_to(1));
            expect!(graph.vertices()).to(be_equal_to(2));
        }

        it "should add edges to a graph" {
            graph.add_edge(1, 2);
            graph.add_edge(3, 4);
            graph.add_edge(3, 2);

            expect!(graph.edges()).to(be_equal_to(3));
            expect!(graph.vertices()).to(be_equal_to(4));
        }

        it "should be adjacent to each other" {
            graph.add_edge(1, 2);

            expect!(graph.adjacent_to(1)).to(be_some().value(&vec![2]));
            expect!(graph.adjacent_to(2)).to(be_some().value(&vec![1]));
        }
    }

    describe! dfs {

        before_each {
            let mut graph = UnidirectedGraph::default();
        }

        it "should create a depth first search" {
            graph.add_edge(1, 2);

            expect!(DepthFirstSearch::new(&graph, 1)).to(be_some());
        }

        it "should not create a depth first search from an empty graph" {
            expect!(DepthFirstSearch::new(&graph, 1)).to(be_none());
        }

        describe! paths {

            before_each {
                graph.add_edge(1, 2);
                graph.add_edge(2, 3);
                graph.add_edge(4, 5);

                let search = DepthFirstSearch::new(&graph, 1).unwrap();
            }

            it "should have path to transient vertices" {
                expect!(search.has_path(3)).to(be_true());
            }

            it "should not have path to not connected vertices" {
                expect!(search.has_path(5)).to(be_false());
            }
        }
    }

    describe! bfs {

        before_each {
            let mut graph = UnidirectedGraph::default();
        }

        it "should create a breadth first search" {
            graph.add_edge(1, 2);

            expect!(BreadthFirstSearch::new(&graph, 1)).to(be_some());
        }

        it "should not create a breadth first search from an empty graph" {
            expect!(BreadthFirstSearch::new(&graph, 1)).to(be_none());
        }

        describe! paths {

            before_each {
                graph.add_edge(1, 2);
                graph.add_edge(2, 3);
                graph.add_edge(4, 5);

                let search = BreadthFirstSearch::new(&graph, 1).unwrap();
            }

            it "should have path to transient vertices" {
                expect!(search.has_path(3)).to(be_true());
            }

            it "should not have path to not connected vertices" {
                expect!(search.has_path(5)).to(be_false());
            }
        }
    }
}
