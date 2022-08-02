use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
/// Time Complexity:    O(`len_rs` * `len_cs` * lg(`k`))
/// Space Complexity:   O(`k`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let len_rs: usize = matrix.len();
        let len_cs: usize = matrix[0].len();
        let k: usize = k as usize;
        let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(len_rs * len_cs);
        for r in 0..len_rs {
            for c in 0..len_cs {
                heap.push(matrix[r][c]);
                if heap.len() > k {
                    heap.pop();
                }
            }
        }
        *heap.peek().unwrap()
    }
}
