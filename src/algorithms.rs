

// QuickSort for BigSize

pub fn quick_sort_with_heap_by<T: Ord + Clone + Copy, F: Fn(&T, &T) -> bool>(array: &mut [T], compare: &F) -> Vec<T> {

    let len = array.len();

    if len < 2 {
        return array.to_vec();
    }

    let pivot = array[len / 2];

    let mut left = Vec::with_capacity(len / 2);
    let mut middle = Vec::with_capacity(len / 2);
    let mut right = Vec::with_capacity(len / 2);

    for i in array {
        if *i < pivot  { left.push(i.clone())   }
        if *i == pivot { middle.push(i.clone()) }
        if *i > pivot  { right.push(i.clone())  }
    }

    let mut left = quick_sort_with_heap_by(&mut left, compare);
    left.extend(middle);
    left.extend(quick_sort_with_heap_by(&mut right, compare));
    left
}

// Classic QuickSort

pub fn quick_sort_without_heap_by<T: Ord + Clone + Copy, F: Fn(&T, &T) -> bool>(array: &mut [T], compare: &F) {

    fn partition<T: Ord, F: Fn(&T, &T) -> bool>(arr: &mut [T], left: isize, right: isize, compare: &F) -> isize {
        let pivot = right;
        let mut i: isize = left as isize - 1;
    
        for j in left..=right - 1 {
            if compare(&arr[j as usize], &arr[pivot as usize]) {
                i += 1;
                arr.swap(i as usize, j as usize);
            }
        }
    
        arr.swap((i + 1) as usize, pivot as usize);
    
        i + 1
    }

    fn _quicksort<T: Ord, F: Fn(&T, &T) -> bool>(arr: &mut [T], left: isize, right: isize, compare: &F) {
        if left <= right {
            let partition_idx = partition(arr, 0, right, compare);
            _quicksort(arr, left, partition_idx - 1, compare);
            _quicksort(arr, partition_idx + 1, right, compare);
        }
    }

    _quicksort(array, 0, (array.len() - 1) as isize, compare);
}





