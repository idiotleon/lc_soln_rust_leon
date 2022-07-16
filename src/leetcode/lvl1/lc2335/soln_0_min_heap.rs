use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/minimum-amount-of-time-to-fill-cups/
/// Time Complexity:    O(max(`nums`) * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fill_cups(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len(); // 3
        let mut heap: BinaryHeap<i32> = {
            let mut heap = BinaryHeap::with_capacity(len_ns);
            for num in nums {
                if num != 0 {
                    heap.push(num);
                }
            }
            heap
        };
        let mut cnt: i32 = 0;
        while let Some(top) = heap.pop() {
            if let Some(sec_top) = heap.pop() {
                if sec_top > 1 {
                    heap.push(sec_top - 1);
                }
            } else {
                return cnt + top;
            }
            if top > 1 {
                heap.push(top - 1);
            }
            cnt += 1;
        }
        cnt
    }
}
