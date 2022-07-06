use std::collections::{HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/jump-game-iii/
/// Time Complexity:     O(`len_ns`)
/// Space Compelxity:    O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/jump-game-iii/discuss/463798/JavaPython-3-BFS-and-DFS-codes-w-analysis.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_reach(nums: Vec<i32>, start: i32) -> bool {
        let len_ns = nums.len();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start as usize);
        let mut seen: HashSet<usize> = HashSet::new();
        seen.insert(start as usize);
        while !queue.is_empty() {
            let len_q = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    if nums[cur] == 0 {
                        return true;
                    }
                    let lo: i32 = cur as i32 - nums[cur];
                    if lo >= 0 && seen.insert(lo as usize) {
                        queue.push_back(lo as usize);
                    }
                    let hi: usize = cur + nums[cur] as usize;
                    if hi < len_ns && seen.insert(hi) {
                        queue.push_back(hi);
                    }
                }
            }
        }
        false
    }
}
