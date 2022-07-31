use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let k: usize = k as usize;
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::with_capacity(len_ns);
        for (idx, num) in nums.into_iter().enumerate() {
            heap.push(Reverse((num, idx)));
            if heap.len() > k {
                heap.pop();
            }
        }
        let mut ans: Vec<(i32, usize)> = Vec::with_capacity(k);
        while let Some(Reverse((num, idx))) = heap.pop() {
            ans.push((num, idx));
        }
        ans.sort_by_key(|e| e.1);
        ans.into_iter().map(|e| e.0).collect()
    }
}
