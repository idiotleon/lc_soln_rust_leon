use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/single-threaded-cpu/description/
/// Time Complexity:    O(`len_ts` * lg(`len_ts`))
/// Space Complexity:   O(`len_ts`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let len_ts: usize = tasks.len();
        let mut hp_tasks: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::with_capacity(len_ts);
        for (idx, task) in tasks.into_iter().enumerate() {
            let time_enqueue = task[0];
            let time_process = task[1];
            hp_tasks.push(Reverse((time_enqueue, time_process, idx as i32)));
        }
        let mut cur_time: i32 = if let Some(&Reverse((enqueue, _process, _id))) = hp_tasks.peek() {
            enqueue
        } else {
            0
        };
        let mut ans: Vec<i32> = Vec::with_capacity(len_ts);
        let mut hp_queue: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::with_capacity(len_ts);
        while !hp_tasks.is_empty() {
            // to enqueue proper tasks
            while let Some(&Reverse((enqueue, process, idx))) = hp_tasks.peek() {
                if enqueue <= cur_time {
                    hp_queue.push(Reverse((process, idx, enqueue)));
                    hp_tasks.pop();
                } else {
                    break;
                }
            }
            // to process one task
            if let Some(Reverse((process, idx, _enqueue))) = hp_queue.pop() {
                ans.push(idx);
                cur_time += process;
            }
            // to wait until the next start,
            // only when there is no task in the processing queue.
            if hp_queue.is_empty() {
                if let Some(&Reverse((enqueue, _process, _idx))) = hp_tasks.peek() {
                    if cur_time < enqueue {
                        cur_time = enqueue;
                    }
                }
            }
        }
        while let Some(Reverse((_process, idx, _enqueue))) = hp_queue.pop() {
            ans.push(idx);
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let tasks: Vec<Vec<i32>> = vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]];
        let actual = Solution::get_order(tasks);
        let expected = vec![0, 2, 3, 1];
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let tasks: Vec<Vec<i32>> =
            vec![vec![7, 10], vec![7, 12], vec![7, 5], vec![7, 4], vec![7, 2]];
        let actual = Solution::get_order(tasks);
        let expected = vec![4, 3, 2, 0, 1];
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_test_case_26_should_return_expected() {
        let tasks: Vec<Vec<i32>> = vec![
            vec![35, 36],
            vec![11, 7],
            vec![15, 47],
            vec![34, 2],
            vec![47, 19],
            vec![16, 14],
            vec![19, 8],
            vec![7, 34],
            vec![38, 15],
            vec![16, 18],
            vec![27, 22],
            vec![7, 15],
            vec![43, 2],
            vec![10, 5],
            vec![5, 4],
            vec![3, 11],
        ];
        let actual = Solution::get_order(tasks);
        let expected = vec![15, 14, 13, 1, 6, 3, 5, 12, 8, 11, 9, 4, 10, 7, 0, 2];
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_test_case_32_should_return_expected() {
        let tasks: Vec<Vec<i32>> = vec![
            vec![5, 2],
            vec![7, 2],
            vec![9, 4],
            vec![6, 3],
            vec![5, 10],
            vec![1, 1],
        ];
        let actual = Solution::get_order(tasks);
        let expected = vec![5, 0, 1, 3, 2, 4];
        assert_eq!(expected, actual);
    }
}
