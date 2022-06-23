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
        let len_n: usize = workers.len();
        const RANGE: i32 = 1e3 as i32 + 1;
        let mut heap: BinaryHeap<Reverse<(u16, usize, usize)>> = {
            let mut heap = BinaryHeap::new();
            for (i, worker) in workers.into_iter().enumerate() {
                for (j, bike) in bikes.iter().enumerate() {
                    let dist: u16 =
                        ((worker[0] - bike[0]).abs() + (worker[1] - bike[1]).abs()) as u16;
                    heap.push(Reverse((dist, i, j)));
                }
            }
            heap
        };
        let mut ans: Vec<i32> = vec![RANGE; len_n];
        let mut assigned: HashSet<usize> = HashSet::new();
        while assigned.len() < len_n {
            if let Some(Reverse(res)) = heap.pop() {
                let (_dist, worker, bike) = res;
                if ans[worker] == RANGE && assigned.insert(bike) {
                    ans[worker] = bike as i32;
                }
            }
        }
        ans
    }
}
