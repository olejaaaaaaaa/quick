use std::time::Instant;

const STACK_SIZE: usize = (1 * 1024 * 1024) / 2; // 0.5 Mb for Stack

mod algorithms;

pub trait QuickSort {
    type Item;
    fn quick_sort(&mut self);
    fn quick_sort_by<F>(&mut self, compare: &F)
    where
        F: Fn(&Self::Item, &Self::Item) -> bool;
}

impl<U: Ord + Clone + Copy> QuickSort for &mut [U] {
    type Item = U;

    fn quick_sort(&mut self) {
        Self::quick_sort_by(self, &|a, b| { b >= a });
    }

    fn quick_sort_by<F>(&mut self, compare: &F)
    where
        F: Fn(&Self::Item, &Self::Item) -> bool,
    {
        if self.len() < 2 { return; }
        if self.len() >= STACK_SIZE {
            algorithms::quick_sort_with_heap_by(self, compare);
        } else {
            algorithms::quick_sort_without_heap_by(self, compare);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn time() {

        let slice: &mut [i32] = &mut [8, 1, 8, 1, 5, 2, 2, 6];
        let time = Instant::now();
        slice.sort();

        println!("Обычная сортировка: {:?}", time.elapsed().as_nanos());

        let mut slice: &mut [i32] = &mut [3, 1, 4, 1, 5, 9, 2, 6];
        let time = Instant::now();
        slice.quick_sort();

        println!("Моя сортировка:     {:?}", time.elapsed().as_nanos());
    }
}