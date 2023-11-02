use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

/// @author: Leon
/// https://leetcode.com/problems/campus-bikes/
/// Time Complexity:    O(`len_n` ^ 2 * lg(`len_n` ^ 2))
/// Space Complexity:   O(`len_n` ^ 2)
/// Reference:
/// https://leetcode.com/problems/campus-bikes/discuss/305603/Java-Fully-Explained
/// https://leetcode.com/problems/campus-bikes/discuss/418060/Step-by-Step-4-solutions-from-600ms-to-14ms-(beating-100)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> Vec<i32> {
        const RANGE: i32 = 1e3 as i32 + 1;
        let len_ws: usize = workers.len();
        let len_bs: usize = bikes.len();
        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = {
            let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> =
                BinaryHeap::with_capacity(len_ws * len_bs);
            for (idx_w, worker) in workers.into_iter().enumerate() {
                for (idx_b, bike) in bikes.iter().enumerate() {
                    let distance = (worker[0] - bike[0]).abs() + (worker[1] - bike[1]).abs();
                    heap.push(Reverse((distance, idx_w, idx_b)));
                }
            }
            heap
        };
        let mut ans: Vec<i32> = vec![RANGE; len_ws];
        let mut assigned: HashSet<usize> = HashSet::with_capacity(len_ws);
        while assigned.len() < len_ws {
            if let Some(Reverse((_distance, idx_w, idx_b))) = heap.pop() {
                if ans[idx_w] == RANGE && assigned.insert(idx_b) {
                    ans[idx_w] = idx_b as i32;
                }
            }
        }
        return ans;
    }
}
