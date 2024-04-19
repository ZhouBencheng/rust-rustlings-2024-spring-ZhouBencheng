/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;
use std::marker::Copy;

pub struct Heap<T>
where
    T: Default + Copy
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Copy
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        let mut cur = self.count;
        let mut parent = self.parent_idx(cur);
        while parent > 0 && (self.comparator)(&self.items[cur - 1], &self.items[parent - 1]) {
            let temp = self.items[cur - 1]; self.items[cur - 1] = self.items[parent - 1]; self.items[parent - 1] = temp;
            cur = parent;
            parent = self.parent_idx(cur);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool { // 监测当前节点是否有孩子
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if !self.children_present(idx) {
            idx
        } else {
            if self.right_child_idx(idx) > self.count || (self.comparator)(&self.items[self.left_child_idx(idx) - 1], &self.items[self.right_child_idx(idx) - 1]) {
                self.left_child_idx(idx)
            } else {
                self.right_child_idx(idx)
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Copy
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone + Copy
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let ret = Some(self.items[0]);
        self.items[0] = self.items[self.count - 1];
        let mut cur = 1;
        let mut next = self.smallest_child_idx(cur);
        while (self.comparator)(&self.items[next - 1], &self.items[cur - 1]) {
            let temp = self.items[cur - 1]; self.items[cur - 1] = self.items[next - 1]; self.items[next - 1] = temp;
            cur = next;
            next = self.smallest_child_idx(cur);
        }
        ret
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Copy,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        println!("{:?}", heap.items);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        println!("{:?}", heap.items);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}