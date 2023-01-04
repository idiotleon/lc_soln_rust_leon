use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/minimum-rounds-to-complete-all-tasks/
/// Time Complexity:    O(`len_ts`)
/// Space Complexity:   O(`len_ts`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let len_ts: usize = tasks.len();
        let task_to_freq: HashMap<i32, u32> = {
            let mut map: HashMap<i32, u32> = HashMap::with_capacity(len_ts);
            for task in tasks {
                *map.entry(task).or_default() += 1;
            }
            map
        };
        let mut cnt: u32 = 0;
        for (_task, freq) in task_to_freq.into_iter() {
            if freq == 1 {
                return -1;
            }
            cnt += freq / 3 + if freq % 3 == 0 { 0 } else { 1 }
        }
        return cnt as i32;
    }
}
