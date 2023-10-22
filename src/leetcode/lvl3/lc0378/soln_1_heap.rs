use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
/// Time Complexity:    O(`_len_rs` * `_len_cs` * lg(`k`))
/// Space Complexity:   O(`k`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let _len_rs: usize = matrix.len();
        let _len_cs: usize = matrix[0].len();
        let k: usize = k as usize;
        let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(k);
        for row in matrix {
            for cur in row {
                heap.push(cur);
                if heap.len() > k {
                    heap.pop();
                }
            }
        }
        *heap.peek().unwrap()
    }
}
