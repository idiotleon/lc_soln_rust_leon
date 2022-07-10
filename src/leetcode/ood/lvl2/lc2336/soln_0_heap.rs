use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

/// @author: Leon
/// https://leetcode.com/problems/smallest-number-in-infinite-set/
/// Time Complexities:
///     `new()`:            O(`RANGE`)
///     `pop_smallest()`:   O(1)
///     `add_back()`:       O(lg(`RANGE`))
/// Space Complexity:       O(`RANGE`)
struct SmallestInfiniteSet {
    heap: BinaryHeap<Reverse<i32>>,
    seen: HashSet<i32>,
}

#[allow(dead_code)]
impl SmallestInfiniteSet {
    fn new() -> Self {
        const RANGE: i32 = 1e3 as i32 + 1;
        Self {
            heap: {
                let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(RANGE as usize);
                for num in 1..RANGE {
                    heap.push(Reverse(num));
                }
                heap
            },
            seen: {
                let mut seen: HashSet<i32> = HashSet::with_capacity(RANGE as usize);
                for num in 1..RANGE {
                    seen.insert(num);
                }
                seen
            },
        }
    }
    fn pop_smallest(&mut self) -> i32 {
        if let Some(Reverse(top)) = self.heap.pop() {
            self.seen.remove(&top);
            return top;
        }
        -1
    }
    fn add_back(&mut self, num: i32) {
        if self.seen.insert(num) {
            self.heap.push(Reverse(num));
        }
    }
}
