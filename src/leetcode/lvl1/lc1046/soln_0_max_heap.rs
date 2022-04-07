use std::collections::BinaryHeap;
/// @author: Leon
/// https://leetcode.com/problems/last-stone-weight/
/// Time Complexity:    O(`len_sts` * lg(`len_sts`))
/// Space Complexity:   O(`len_sts`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i16> = {
            let mut heap = BinaryHeap::new();
            for st in stones {
                heap.push(st as i16);
            }
            heap
        };
        while heap.len() > 1 {
            let top1 = heap.pop().unwrap();
            let top2 = heap.pop().unwrap();
            let hit = (top1 - top2).abs();
            if hit > 0 {
                heap.push(hit);
            }
        }
        if heap.is_empty() {
            0
        } else {
            heap.pop().unwrap() as i32
        }
    }
}
