/// https://leetcode.com/problems/kth-largest-element-in-an-array/
/// 
/// Time Complexity:    O()
/// Space Complexity:   O()
/// 
/// References:
///     https://leetcode.com/problems/kth-largest-element-in-an-array/discuss/950796/Rust-BinaryHeap-0ms
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();

        for num in nums {
            max_heap.push(num)
        }

        let mut k = k;
        while k > 1{
            max_heap.pop();
            k -= 1;
        }

        *max_heap.peek().unwrap()
    }
}