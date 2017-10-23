pub use tdd_kata::mst_kata::day_1::{EdgeWeightedGraph, WeightedEdge, LazyMst};

pub use std::cmp::Ordering;

pub use expectest::prelude::{be_equal_to, be_some, be_ok, be_err};

describe! edge_weighted_graph {

    describe! weighted_edge {

        before_each {
            let edge = WeightedEdge::new(1, 2, 1);
        }

        it "should create a new edge with weight" {
            expect!(edge.weight()).to(be_equal_to(1));
        }

        it "should return other vertex" {
            expect!(edge.either()).to(be_equal_to(1));
            expect!(edge.other(edge.either())).to(be_equal_to(2));
            expect!(edge.other(edge.other(edge.either()))).to(be_equal_to(1));
        }

        it "should be comparable by weight" {
            let bigger = WeightedEdge::new(1, 2, 3);
            let smaller = WeightedEdge::new(2, 3, 1);

            expect!(bigger.cmp(&smaller)).to(be_equal_to(Ordering::Less));
            expect!(smaller.cmp(&bigger)).to(be_equal_to(Ordering::Greater));

            let edge_1 = WeightedEdge::new(1, 2, 2);
            let edge_2 = WeightedEdge::new(1, 3, 2);

            expect!(edge_1.cmp(&edge_2)).to(be_equal_to(Ordering::Equal));
        }
    }

    describe! graph {

        before_each {
            let mut graph = EdgeWeightedGraph::default();
        }

        it "should create a new empty edge weighted graph" {
            expect!(graph.vertices()).to(be_equal_to(0));
            expect!(graph.edges()).to(be_equal_to(0));
        }

        it "should add a weighted edge to a graph" {
            graph.add_edge(WeightedEdge::new(1, 2, 1));

            expect!(graph.vertices()).to(be_equal_to(2));
            expect!(graph.edges()).to(be_equal_to(1));
        }

        it "should add weighted edges to a graph" {
            graph.add_edge(WeightedEdge::new(1, 2, 1));
            graph.add_edge(WeightedEdge::new(2, 3, 1));
            graph.add_edge(WeightedEdge::new(1, 4, 1));

            expect!(graph.vertices()).to(be_equal_to(4));
            expect!(graph.edges()).to(be_equal_to(3));
        }

        it "should be adjacent to each other" {
            let edge = WeightedEdge::new(1, 2, 1);

            graph.add_edge(edge);

            expect!(graph.adjacent_to(1)).to(be_some().value(&vec![edge]));
            expect!(graph.adjacent_to(2)).to(be_some().value(&vec![edge]));
        }
    }

    describe! lazy_mst {

        it "should create a lazy mst" {
            let mut graph = EdgeWeightedGraph::default();

            graph.add_edge(WeightedEdge::new(1, 2, 1));
            graph.add_edge(WeightedEdge::new(2, 3, 1));
            graph.add_edge(WeightedEdge::new(1, 4, 1));

            expect!(LazyMst::new(&graph)).to(be_ok());
        }

        it "should not create a lazy mst from an empty graph" {
            let graph = EdgeWeightedGraph::default();

            expect!(LazyMst::new(&graph)).to(be_err());
        }

        it "should be weight of 3" {
            let mut graph = EdgeWeightedGraph::default();

            graph.add_edge(WeightedEdge::new(1, 2, 1));
            graph.add_edge(WeightedEdge::new(2, 3, 1));
            graph.add_edge(WeightedEdge::new(1, 4, 1));

            let mst = LazyMst::new(&graph).unwrap();

            expect!(mst.weight()).to(be_equal_to(3));
        }

        it "should be weight of 2" {
            let mut graph = EdgeWeightedGraph::default();

            graph.add_edge(WeightedEdge::new(1, 2, 1));
            graph.add_edge(WeightedEdge::new(2, 3, 1));
            graph.add_edge(WeightedEdge::new(1, 3, 1));

            let mst = LazyMst::new(&graph).unwrap();

            expect!(mst.weight()).to(be_equal_to(2));
        }
    }
}