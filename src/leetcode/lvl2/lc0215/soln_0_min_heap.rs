use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// @author: Leon
/// https://leetcode.com/problems/kth-largest-element-in-an-array/
/// Time Complexity:    O(`_len_n` * lg(`k`))
/// Space Complexity:   O(`k`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let _len_n = nums.len();
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for num in nums {
            heap.push(Reverse(num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.peek().unwrap().0
    }
}
