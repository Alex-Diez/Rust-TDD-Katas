#![allow(not_unsafe_ptr_arg_deref)]

use std::ptr;
use std::mem;

pub trait Sort<T: Ord> {

    fn sort(data: &mut [T]);
}

pub struct BubbleSort;

impl <T: Ord> Sort<T> for BubbleSort {

    fn sort(data: &mut[T]) {
        let len = data.len();
        unsafe {
            let data_ptr = data.as_mut_ptr();
            for i in 0..len {
                let pass = len - i - 1;
                for (j, item) in data.iter().enumerate().take(pass) {
                    if *item > data[j+1] {
                        ptr::swap(data_ptr.offset(j as isize), data_ptr.offset((j+1) as isize));
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
                    let tmp = ptr::read(key);
                    ptr::copy(&*data_ptr.offset(j), data_ptr.offset(j + 1), (i - j) as usize);
                    ptr::copy_nonoverlapping(&tmp, data_ptr.offset(j), 1);
                    mem::forget(tmp);
                }
            }
        }
    }
}

pub struct InPlaceMergeSort;

impl InPlaceMergeSort {

    fn sort_inner<T: Ord>(data: *mut T, auxilary: *mut T, low: usize, high: usize) {
        if high > low {
            let middle = low + (high - low) / 2;
            InPlaceMergeSort::sort_inner(data, auxilary, low, middle);
            InPlaceMergeSort::sort_inner(data, auxilary, middle + 1, high);
            InPlaceMergeSort::merge(data, auxilary, low, middle, high);
        }
    }

    fn merge<T: Ord>(data: *mut T, auxilary: *mut T, low: usize, middle: usize, high: usize) {
        let low = low as isize;
        let middle = middle as isize;
        let high = high as isize;
        let mut i = low;
        let mut j = middle + 1;
        unsafe {
            ptr::copy(data.offset(low), auxilary.offset(low), (high - low + 1) as usize);
            for k in low..high+1 {
                if i > middle {
                    ptr::copy(auxilary.offset(j), data.offset(k), 1);
                    j += 1;
                }
                else if j > high {
                    ptr::copy(auxilary.offset(i), data.offset(k), 1);
                    i += 1;
                }
                else if &*auxilary.offset(j) < &*auxilary.offset(i) {
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

impl <T: Ord> Sort<T> for InPlaceMergeSort {

    fn sort(data: &mut [T]) {
        let data_ptr = data.as_mut_ptr();
        let len = data.len();
        let mut auxilary = Vec::with_capacity(len);
        let auxilary_ptr = auxilary.as_mut_ptr();
        InPlaceMergeSort::sort_inner(data_ptr, auxilary_ptr, 0, len - 1);
    }
}
