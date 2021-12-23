/// https://leetcode.com/problems/kth-largest-element-in-an-array/
/// Time Complexity:    O(`_len_n` * lg(`k`))
/// Space Complexity:   O(`k`)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let _len_n = nums.len();
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for num in nums {
            min_heap.push(Reverse(num));
            if min_heap.len() > k as usize {
                min_heap.pop();
            }
        }
        min_heap.peek().unwrap().0
    }
}
