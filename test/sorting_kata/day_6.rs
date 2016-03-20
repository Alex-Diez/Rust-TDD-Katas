pub use tdd_kata::sorting_kata::day_6::{Sort, BubbleSort, InsertSort, TopDownMergeSort, BottomUpMergeSort, QuickSort};

pub use rand;
pub use rand::distributions::Sample;
pub use rand::distributions::range::Range;

pub use expectest::prelude::be_equal_to;

describe! array {

    before_each {
        const SIZE: usize = 10;
        let mut between = Range::new(0, 1000);
        let mut rng = rand::thread_rng();

        let mut data = Vec::with_capacity(SIZE);
        let mut ret = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            let datum = between.sample(&mut rng);
            data.push(datum);
            ret.push(datum);
        }
        ret.sort();
    }

    it "should be sorted out by bubble sort" {
        BubbleSort::sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }

    it "should be sorted out by insert sort" {
        InsertSort::sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }

    it "should be sorted out by top down merge sort" {
        TopDownMergeSort::sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }

    it "should be sorted out by bottom up merge sort" {
        BottomUpMergeSort::sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }

    it "should be sorted out by quick sort" {
        QuickSort::sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }
}
