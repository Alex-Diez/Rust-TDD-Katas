pub use tdd_kata::sorting_kata::day_10::{bubble_sort, insert_sort, top_down_merge_sort, bottom_up_merge_sort, quick_sort};

pub use rand;
pub use rand::distributions::range::Range;
pub use rand::distributions::Sample;

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

    it "should be sorted out by bubble sort" {
        bubble_sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }

    it "should be sorted out by insert sort" {
        insert_sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }

    it "should be sorted out by top down merge sort" {
        top_down_merge_sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }

    it "should be sorted out by bottom up merge sort" {
        bottom_up_merge_sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }

    it "should be sorted out by quick sort" {
        quick_sort(&mut data);
        expect!(data).to(be_equal_to(ret));
    }
}
