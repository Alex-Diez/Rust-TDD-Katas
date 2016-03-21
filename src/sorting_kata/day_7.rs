use std::ptr;
use std::mem;
use std::cmp;
use rand::{Rng, self};

pub fn bubble_sort<T: Ord>(data: &mut [T]) {
    let len = data.len();
    let data_ptr = data.as_mut_ptr();
    for i in 0..len {
        let pass = len - i - 1;
        for (j, item) in data.iter().enumerate().take(pass) {
            if *item > data[j + 1] {
                unsafe {
                    ptr::swap(data_ptr.offset(j as isize), data_ptr.offset((j + 1) as isize));
                }
            }
        }
    }
}

pub fn insert_sort<T: Ord>(data: &mut [T]) {
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

pub fn top_down_merge_sort<T: Ord>(data: &mut [T]) {
    let len = data.len();
    let mut auxilary = Vec::with_capacity(len);
    let data_ptr = data.as_mut_ptr();
    let auxilary_ptr = auxilary.as_mut_ptr();
    merge_sort(data_ptr, auxilary_ptr, 0, (len - 1) as isize);
}

fn merge_sort<T: Ord>(data: *mut T, auxilary: *mut T, low: isize, high: isize) {
    if high > low {
        let middle = low + (high - low) / 2;
        merge_sort(data, auxilary, low, middle);
        merge_sort(data, auxilary, middle + 1, high);
        merge(data, auxilary, low, middle, high);
    }
}

fn merge<T: Ord>(data: *mut T, auxilary: *mut T, low: isize, middle: isize, high: isize) {
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

pub fn bottom_up_merge_sort<T: Ord>(data: &mut [T]) {
    let len = data.len() as isize;
    let mut auxilary = Vec::with_capacity(len as usize);
    let data_ptr = data.as_mut_ptr();
    let auxilary_ptr = auxilary.as_mut_ptr();
    let mut sz = 1;
    while sz < len {
        let mut low = 0;
        while low < len - sz {
            merge(data_ptr, auxilary_ptr, low, low + sz - 1, cmp::min(low + sz + sz - 1, len - 1));
            low += sz + sz;
        }
        sz += sz;
    }
}

pub fn quick_sort<T: Ord>(data: &mut [T]) {
    let mut rng = rand::thread_rng();
    rng.shuffle(data);
    let data_ptr = data.as_mut_ptr();
    let len = data.len() as isize;
    quick_sort_inner(data_ptr, 0, len - 1);
}

fn quick_sort_inner<T: Ord>(data: *mut T, low: isize, high: isize) {
    if high > low {
        let part = partition(data, low, high);
        quick_sort_inner(data, low, part - 1);
        quick_sort_inner(data, part + 1, high);
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
