use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/remove-stones-to-minimize-the-total/
/// Time Complexity:    O(`k` * lg(`_len_ps`))
/// Space Complexity:   O(`_len_ps`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let _len_ps = piles.len();
        let mut max_heap = BinaryHeap::<i32>::new();
        for pile in piles {
            max_heap.push(pile);
        }
        let mut k = k;
        while k > 0 {
            if let Some(top) = max_heap.pop() {
                let removed = top - top / 2;
                max_heap.push(removed);
            }
            k -= 1;
        }
        max_heap.iter().sum()
    }
}
