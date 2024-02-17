use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/furthest-building-you-can-reach/
/// Time Complexity:     O(`len_bs` * lg(`len_bs`))
/// Space Complexity:    O(`len_bs`)
/// Reference:
/// https://leetcode.com/problems/furthest-building-you-can-reach/discuss/1177681/Rust-BinaryHeap-solution
/// https://leetcode.com/problems/furthest-building-you-can-reach/discuss/918515/JavaC%2B%2BPython-Priority-Queue
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut bricks = bricks;
        let len_bs = heights.len();
        let mut min_heap = BinaryHeap::with_capacity(len_bs);
        for i in 0..len_bs - 1 {
            let diff = heights[i + 1] - heights[i];
            if diff > 0 {
                min_heap.push(-diff);
            }
            if min_heap.len() > ladders as usize {
                if let Some(min) = min_heap.pop() {
                    bricks += min;
                    if bricks < 0 {
                        return i as i32;
                    }
                }
            }
        }
        len_bs as i32 - 1
    }
}
