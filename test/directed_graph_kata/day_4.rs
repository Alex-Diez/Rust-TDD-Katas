pub use tdd_kata::directed_graph_kata::day_4::{DirectedGraph, DirectedCycle};

pub use expectest::prelude::{be_equal_to, be_some, be_none, be_ok, be_err, be_true, be_false};

describe! directed_graph {

    before_each {
        let mut graph = DirectedGraph::default();
    }

    describe! graph {

        it "should create a new empty graph" {
            expect!(graph.vertices()).to(be_equal_to(0));
            expect!(graph.edges()).to(be_equal_to(0));
        }

        it "should add an edge to a graph" {
            graph.add_edge(1, 2);

            expect!(graph.vertices()).to(be_equal_to(2));
            expect!(graph.edges()).to(be_equal_to(1));
        }

        it "should add edges to a graph" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(1, 4);

            expect!(graph.vertices()).to(be_equal_to(4));
            expect!(graph.edges()).to(be_equal_to(3));
        }

        it "should not be adjacent to each other" {
            graph.add_edge(1, 2);

            expect!(graph.adjacent_to(1)).to(be_some().value(&vec![2]));
            expect!(graph.adjacent_to(2)).to(be_some().value(&vec![]));
        }
    }

    describe! directed_cycle {

        it "should create a directed cycle" {
            graph.add_edge(1, 2);

            expect!(DirectedCycle::new(&graph)).to(be_ok());
        }

        it "should not create a directed cycle from an empty graph" {
            expect!(DirectedCycle::new(&graph)).to(be_err());
        }

        it "should have cycle" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(3, 1);

            let cycle = DirectedCycle::new(&graph).unwrap();

            expect!(cycle.has_cycle()).to(be_true());
        }

        it "should not have cycle" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(1, 4);

            let cycle = DirectedCycle::new(&graph).unwrap();

            expect!(cycle.has_cycle()).to(be_false());
        }
    }
}
