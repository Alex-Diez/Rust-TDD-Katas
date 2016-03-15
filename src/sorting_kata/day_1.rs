use std::ptr;

pub trait Sort<T: Ord> {

    fn sort(&self, data: &mut [T]);
}

pub struct BubbleSort;

impl Default for BubbleSort {

    fn default() -> BubbleSort {
        BubbleSort
    }
}

impl <T: Ord> Sort<T> for BubbleSort {

    fn sort(&self, data: &mut [T]) {
        unsafe {
            let ptr = data.as_mut_ptr();
            let len = data.len();
            for i in 0..len {
                let pass = len - 1 - i;
                for j in 0..pass {
                    if data[j] > data[j + 1] {
                        ptr::swap(ptr.offset(j as isize), ptr.offset((j+1) as isize));
                    }
                }
            }
        }
    }
}

pub struct InsertSort;

impl Default for InsertSort {

    fn default() -> InsertSort {
        InsertSort
    }
}

impl <T: Ord> Sort<T> for InsertSort {

    fn sort(&self, data: &mut[T]) {
        unsafe {
            let ptr = data.as_mut_ptr();
            for i in 0..data.len() {
                let mut j = i + 1;
                while j < data.len() {
                    if data[i] > data[j] {
                        ptr::swap(ptr.offset(i as isize), ptr.offset(j as isize));
                    }
                    j += 1;
                }
            }
        }
    }
}

pub struct InPlaceMergeSort;

impl InPlaceMergeSort {

    unsafe fn inner_sort<T: Ord>(&self, data: *mut T, auxilary: *mut T, low: usize, high: usize) {
        if high > low {
            let middle = low + (high - low) / 2;
            self.inner_sort(data, auxilary, low, middle);
            self.inner_sort(data, auxilary, middle + 1, high);
            self.merge(data, auxilary, low, middle, high);
        }
    }

    unsafe fn merge<T: Ord>(&self, data: *mut T, auxilary: *mut T, low: usize, middle: usize, high: usize) {
        let mut i = low;
        let mut j = middle + 1;
        ptr::copy(data.offset(low as isize), auxilary.offset(low as isize), high - low + 1);
        println!("low - {:?} middle - {:?} high - {:?}", low, middle, high);
        for k in low..high {
            println!("k - {:?} i - {:?} j - {:?}", k, i, j);
            if i > middle {
                ptr::copy(auxilary.offset(j as isize), data.offset(k as isize), 1);
                j += 1;
            }
            else if j > high {
                ptr::copy(auxilary.offset(i as isize), data.offset(k as isize), 1);
                i += 1;
            }
            else if auxilary.offset(j as isize) < auxilary.offset(i as isize) {
                ptr::copy(auxilary.offset(j as isize), data.offset(k as isize), 1);
                j += 1;
            }
            else {
                ptr::copy(auxilary.offset(i as isize), data.offset(k as isize), 1);
                i += 1;
            }
        }
    }
}

impl Default for InPlaceMergeSort {

    fn default() -> InPlaceMergeSort {
        InPlaceMergeSort
    }
}

impl <T: Ord> Sort<T> for InPlaceMergeSort {

    fn sort(&self, data: &mut [T]) {
        let len = data.len();
        let mut auxilary = Vec::with_capacity(len);
        unsafe {
            let data_ptr = data.as_mut_ptr();
            let auxilary_ptr = auxilary.as_mut_ptr();
            self.inner_sort(data_ptr, auxilary_ptr, 0, len - 1);
        }
    }
}
