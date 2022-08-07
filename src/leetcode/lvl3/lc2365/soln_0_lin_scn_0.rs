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
            latest = std::cmp::max(*task_to_exp.get(&task).unwrap_or(&0), latest + 1);
            task_to_exp.insert(task, latest + space + 1);
        }
        return latest;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let tasks: Vec<i32> = vec![1, 2, 1, 2, 3, 1];
        let space: i32 = 3;
        let expected: i64 = 9;
        let actual = Solution::task_scheduler_ii(tasks, space);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let tasks: Vec<i32> = vec![5, 8, 8, 5];
        let space: i32 = 2;
        let expected: i64 = 6;
        let actual = Solution::task_scheduler_ii(tasks, space);
        assert_eq!(expected, actual);
    }
}
