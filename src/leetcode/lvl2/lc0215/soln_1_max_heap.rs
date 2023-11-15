use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/kth-largest-element-in-an-array/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
/// References:
/// https://leetcode.com/problems/kth-largest-element-in-an-array/discuss/950796/Rust-BinaryHeap-0ms
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::with_capacity(len_ns);
        for num in nums {
            max_heap.push(num)
        }
        let mut k = k;
        while k > 1 {
            max_heap.pop();
            k -= 1;
        }
        *max_heap.peek().unwrap()
    }
}
