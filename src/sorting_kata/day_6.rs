use std::ptr;
use std::mem;
use std::cmp;
use rand::{Rng, self};

pub trait Sort<T: Ord> {

    fn sort(data: &mut [T]);
}

pub struct BubbleSort;

impl <T: Ord> Sort<T> for BubbleSort {

    fn sort(data: &mut [T]) {
        let len = data.len();
        let data_ptr = data.as_mut_ptr();
        for i in 0..len {
            let pass = len - i - 1;
            for (j, item) in data.iter().enumerate().take(pass) {
                if *item > data[j+1] {
                    unsafe {
                        ptr::swap(data_ptr.offset(j as isize), data_ptr.offset((j + 1) as isize));
                    }
                }
            }
        }
    }
}

pub struct InsertSort;

impl <T: Ord> Sort<T> for InsertSort {

    fn sort(data: &mut [T]) {
        let len = data.len() as isize;
        let data_ptr = data.as_mut_ptr();
        for i in 1..len {
            let mut j = i;
            unsafe {
                let key = data_ptr.offset(i);
                while j > 0 && &*data_ptr.offset(j - 1) > &*key {
                    j -= 1;
                }
                if i != j {
                    let temp = ptr::read(key);
                    ptr::copy(&*data_ptr.offset(j), data_ptr.offset(j + 1), (i - j) as usize);
                    ptr::copy_nonoverlapping(&temp, data_ptr.offset(j), 1);
                    mem::forget(temp);
                }
            }
        }
    }
}

pub trait MergeSort {

    fn merge<T: Ord>(data: *mut T, auxilary: *mut T, low: isize, middle: isize, high: isize) {
        let mut i = low;
        let mut j = middle + 1;
        unsafe {
            ptr::copy(data.offset(low), auxilary.offset(low), (high - low + 1) as usize);
            for k in low..high + 1 {
                if i > middle {
                    ptr::copy(auxilary.offset(j), data.offset(k), 1);
                    j += 1;
                }
                else if j > high {
                    ptr::copy(auxilary.offset(i), data.offset(k), 1);
                    i += 1;
                }
                else if &*auxilary.offset(i) > &*auxilary.offset(j) {
                    ptr::copy(auxilary.offset(j), data.offset(k), 1);
                    j += 1;
                }
                else {
                    ptr::copy(auxilary.offset(i), data.offset(k), 1);
                    i += 1;
                }
            }
        }
    }
}

pub struct TopDownMergeSort;

impl TopDownMergeSort {

    fn sort_inner<T: Ord>(data: *mut T, auxilary: *mut T, low: isize, high: isize) {
        if high > low {
            let middle = low + (high - low) / 2;
            TopDownMergeSort::sort_inner(data, auxilary, low, middle);
            TopDownMergeSort::sort_inner(data, auxilary, middle + 1, high);
            TopDownMergeSort::merge(data, auxilary, low, middle, high);
        }
    }
}

impl MergeSort for TopDownMergeSort { }

impl <T: Ord> Sort<T> for TopDownMergeSort {

    fn sort(data: &mut [T]) {
        let len = data.len();
        let mut auxilary = Vec::with_capacity(len);
        TopDownMergeSort::sort_inner(data.as_mut_ptr(), auxilary.as_mut_ptr(), 0, (len - 1) as isize);
    }
}

pub struct BottomUpMergeSort;

impl MergeSort for  BottomUpMergeSort { }

impl <T: Ord> Sort<T> for BottomUpMergeSort {

    fn sort(data: &mut [T]) {
        let len = data.len() as isize;
        let mut auxilary = Vec::with_capacity(len as usize);
        let auxilary_ptr = auxilary.as_mut_ptr();
        let data_ptr = data.as_mut_ptr();
        let mut sz = 1;
        while sz < len {
            let mut low = 0;
            while low < len - sz {
                BottomUpMergeSort::merge(data_ptr, auxilary_ptr, low, low + sz - 1, cmp::min(low + sz + sz - 1, len - 1));
                low += sz + sz;
            }
            sz += sz;
        }
    }
}

pub struct QuickSort;

impl QuickSort {

    fn sort_inner<T: Ord>(data: *mut T, low: isize, high: isize) {
        if high > low {
            let part = QuickSort::partition(data, low, high);
            QuickSort::sort_inner(data, low, part - 1);
            QuickSort::sort_inner(data, part + 1, high);
        }
    }

    fn partition<T: Ord>(data: *mut T, low: isize, high: isize) -> isize {
        let mut i = low + 1;
        let mut j = high;
        unsafe {
            let value = data.offset(low);
            loop {
                while i != high && &*data.offset(i) < &*value {
                    i += 1;
                }
                while j != low && &*data.offset(j) > &*value {
                    j -= 1;
                }
                if i >= j {
                    break;
                }
                ptr::swap(data.offset(i), data.offset(j));
            }
            ptr::swap(data.offset(low), data.offset(j));
        }
        j
    }
}

impl <T: Ord> Sort<T> for QuickSort {

    fn sort(data: &mut [T]) {
        let mut rng = rand::thread_rng();
        rng.shuffle(data);
        let data_ptr = data.as_mut_ptr();
        let len = data.len() as isize;
        QuickSort::sort_inner(data_ptr, 0, len - 1);
    }
}
