/// https://leetcode.com/problems/furthest-building-you-can-reach/
///
/// Time Complexity:     O(`size` * lg(`size`))
/// Space Complexity:    O(`size`)
///
/// Reference:
/// https://leetcode.com/problems/furthest-building-you-can-reach/discuss/1177681/Rust-BinaryHeap-solution
/// https://leetcode.com/problems/furthest-building-you-can-reach/discuss/918515/JavaC%2B%2BPython-Priority-Queue
use std::collections::BinaryHeap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut bricks = bricks;
        let size = heights.len();
        let mut min_heap = BinaryHeap::new();

        for i in 0..size - 1 {
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
        size as i32 - 1
    }
}
