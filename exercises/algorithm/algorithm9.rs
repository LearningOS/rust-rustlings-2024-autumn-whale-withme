/*
	heap
	This question requires you to implement a binary heap function
*/
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Clone, // Add Clone trait bound
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone, // Add Clone trait bound
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // Starting with a dummy element at index 0
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
        // Add the new value to the end of the heap
        self.count += 1;
        if self.count >= self.items.len() {
            self.items.push(value); // Expand the vector if needed
        } else {
            self.items[self.count] = value; // Replace the value at the current position
        }
        self.bubble_up(self.count); // Restore heap property
    }

    fn bubble_up(&mut self, idx: usize) {
        let mut idx = idx;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx); // Swap the current node with its parent
                idx = parent_idx; // Move up the tree
            } else {
                break; // If heap property is satisfied, stop
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        if right_idx <= self.count && (self.comparator)(&self.items[right_idx], &self.items[left_idx]) {
            right_idx // Return the index of the smaller child
        } else {
            left_idx // Return the index of the left child
        }
    }

    pub fn remove(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Heap is empty");
        }

        let root_value = self.items[1].clone(); // Take the root value
        self.items[1] = self.items[self.count].clone(); // Move the last item to the root
        self.count -= 1; // Decrease the count
        self.bubble_down(1); // Restore heap property
        Ok(root_value)
    }

    fn bubble_down(&mut self, idx: usize) {
        let mut idx = idx;
        while self.children_present(idx) {
            let child_idx = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                self.items.swap(idx, child_idx); // Swap with the smaller child
                idx = child_idx; // Move down the tree
            } else {
                break; // If heap property is satisfied, stop
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone, // Add Clone trait bound for Ord implementation
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
    T: Default + Clone, // Ensure T can be cloned for iteration
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove().unwrap()) // Remove and return the root
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone, // Add Clone trait bound
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone, // Add Clone trait bound
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
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
