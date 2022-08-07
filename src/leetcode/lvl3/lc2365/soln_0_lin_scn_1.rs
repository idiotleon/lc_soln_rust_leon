use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/task-scheduler-ii/
/// Time Complexity:    O(`len_ts`)
/// Space Complexity:   O(`len_ts`)
/// Reference:
/// https://leetcode.com/problems/task-scheduler-ii/discuss/2388301/JavaC%2B%2BPython-HashMap
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let len_ts: usize = tasks.len();
        let space: i64 = space as i64;
        let mut task_to_exp: HashMap<i32, i64> = HashMap::with_capacity(len_ts);
        let mut latest: i64 = 0;
        for task in tasks {
            match task_to_exp.get(&task) {
                Some(&last_exp) => {
                    latest = std::cmp::max(latest, last_exp + space) + 1;
                }
                None => {
                    latest += 1;
                }
            };
            task_to_exp.insert(task, latest);
        }
        return latest;
    }
}
