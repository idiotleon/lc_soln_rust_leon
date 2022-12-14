use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

/// @author: Leon
/// https://leetcode.com/problems/campus-bikes-ii/description/
/// Time Complexity:    O(V + E * lg(V))
/// Space Complexity:   O(E * lg(V))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        let len_ws: usize = workers.len();
        let len_bs: usize = bikes.len();
        let mut heap: BinaryHeap<Reverse<(i32, usize, u32)>> = {
            let mut heap = BinaryHeap::new();
            heap.push(Reverse((0, 0, 0)));
            heap
        };
        let mut seen: HashSet<String> = HashSet::new();
        while let Some(Reverse((distance, idx_w, state))) = heap.pop() {
            let hash: String = format!("{}#{}", idx_w, state);
            if !seen.insert(hash) {
                continue;
            }
            if idx_w == len_ws {
                return distance;
            }
            for idx_b in 0..len_bs {
                if state & (1 << idx_b) == 0 {
                    heap.push(Reverse((
                        distance + Self::get_distance(&workers[idx_w], &bikes[idx_b]),
                        idx_w + 1,
                        state | (1 << idx_b),
                    )));
                }
            }
        }
        return -1;
    }
    fn get_distance(worker: &Vec<i32>, bike: &Vec<i32>) -> i32 {
        return (worker[0] - bike[0]).abs() + (worker[1] - bike[1]).abs();
    }
}
