use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/kth-largest-element-in-an-array/
/// Time Complexity:    O(`len_ns` * lg(`k`))
/// Space Complexity:   O(`k`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns = nums.len();
        let k: usize = k as usize;
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(len_ns);
        for num in nums {
            heap.push(Reverse(num));
            if heap.len() > k {
                heap.pop();
            }
        }
        if let Some(Reverse(top)) = heap.pop() {
            return top;
        } else {
            unreachable!()
        }
    }
}
