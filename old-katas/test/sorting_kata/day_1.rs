pub use tdd_kata::sorting_kata::day_1::{Sort, BubbleSort, InsertSort, InPlaceMergeSort};

pub use rand::distributions::range::Range;
pub use rand::distributions::Sample;
pub use rand;
pub use expectest::prelude::be_equal_to;

describe! array {

    before_each {
        let mut between = Range::new(0, 1000);
        let mut rng = rand::thread_rng();

        let mut data = Vec::with_capacity(10);
        let mut ret = Vec::with_capacity(10);
        for _ in 0..10 {
            let datum = between.sample(&mut rng);
            data.push(datum);
            ret.push(datum);
        }
        ret.sort();
    }

    it "should be sorted out by bubble algorithm" {
        let bubble_sort = BubbleSort::default();
        bubble_sort.sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }

    it "should be sorted out by insert algorithm" {
        let insert_sort = InsertSort::default();
        insert_sort.sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }

    ignore "should be sorted out by merge algorithm" {
        let merge_sort = InPlaceMergeSort::default();
        merge_sort.sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }
}
