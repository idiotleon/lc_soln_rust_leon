// https://leetcode.com/problems/jump-game-iii/
//
// Time Complexity:     O(`len_arr`)
// Space Compelxity:    O(`len_arr`)
//
// Reference:
//  https://leetcode.com/problems/jump-game-iii/discuss/463798/JavaPython-3-BFS-and-DFS-codes-w-analysis.
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let len_arr = arr.len();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start as usize);
        let mut seen: HashSet<usize> = HashSet::new();
        seen.insert(start as usize);
        while !queue.is_empty() {
            let len_lvl = queue.len();
            for _ in 0..len_lvl {
                if let Some(cur) = queue.pop_front() {
                    if arr[cur] == 0 {
                        return true;
                    }
                    let lo: i32 = cur as i32 - arr[cur];
                    if lo >= 0 && seen.insert(lo as usize) {
                        queue.push_back(lo as usize);
                    }
                    let hi: usize = cur + arr[cur] as usize;
                    if hi < len_arr && seen.insert(hi) {
                        queue.push_back(hi);
                    }
                }
            }
        }
        false
    }
}
