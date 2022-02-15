use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/sort-even-and-odd-indices-independently/
/// Time Complexity:    O(`len_n` * lg(`len_n`))
/// Space Complexity:   O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/sort-even-and-odd-indices-independently/discuss/1748447/Heap(O(n-log-n))-and-Counting-Sort(O(n))-in-Java
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        // odd indices: max heap
        let mut odd: BinaryHeap<i32> = BinaryHeap::new();
        // even indices: min heap
        let mut even: BinaryHeap<i32> = BinaryHeap::new();
        for (idx, num) in nums.into_iter().enumerate() {
            if idx % 2 == 0 {
                even.push(-num);
            } else {
                odd.push(num);
            }
        }
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = Vec::new();
            for idx in 0..len_n {
                if idx % 2 == 0 {
                    if let Some(top) = even.pop() {
                        res.push(-top);
                    }
                } else {
                    if let Some(top) = odd.pop() {
                        res.push(top);
                    }
                }
            }
            res
        };
        ans
    }
}
