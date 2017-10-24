pub use tdd_kata::directed_graph_kata::day_1::{DirectedGraph, DirectedCycle, TopologySort};

pub use expectest::prelude::{be_equal_to, be_some, be_none, be_ok, be_err, be_true, be_false};

describe! directed_graph {

    before_each {
        let mut graph = DirectedGraph::default();
    }

    describe! graph {

        it "should create a new empty directed graph" {
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
            graph.add_edge(3, 1);

            expect!(graph.vertices()).to(be_equal_to(3));
            expect!(graph.edges()).to(be_equal_to(3));
        }

        it "should not adjacent to each other" {
            graph.add_edge(1, 2);

            expect!(graph.adjacent_to(1)).to(be_some().value(&vec![2]));
            expect!(graph.adjacent_to(2)).to(be_none());
        }
    }

    describe! cycles {

        it "should create a directed cycle" {
            graph.add_edge(1, 2);

            expect!(DirectedCycle::new(&graph)).to(be_ok());
        }

        it "should not create a directed cycle from an empty graph" {
            expect!(DirectedCycle::new(&graph)).to(be_err());
        }

        it "should have a cycle" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(3, 1);

            let cycle = DirectedCycle::new(&graph).unwrap();

            expect!(cycle.has_cycle()).to(be_true());
        }

        it "should not have a cycle" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(1, 4);

            let cycle = DirectedCycle::new(&graph).unwrap();

            expect!(cycle.has_cycle()).to(be_false());
        }
    }

    describe! topology_sort {

        it "should create a topology sort" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);

            expect!(TopologySort::new(&graph)).to(be_ok());
        }

        it "should not create a topology sort from empty graph" {
            expect!(TopologySort::new(&graph)).to(be_err());
        }

        it "should be an acyclic graph" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(1, 4);

            let topology_sort = TopologySort::new(&graph).unwrap();

            expect!(topology_sort.is_acyclic_graph()).to(be_true());
        }

        it "should not be an acyclic graph" {
            graph.add_edge(1, 2);
            graph.add_edge(2, 3);
            graph.add_edge(3, 1);

            let topology_sort = TopologySort::new(&graph).unwrap();

            expect!(topology_sort.is_acyclic_graph()).to(be_false());
        }
    }
}
