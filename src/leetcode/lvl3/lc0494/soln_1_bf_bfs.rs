use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/target-sum/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let len_ns: usize = nums.len();
        let mut lvl: usize = 1;
        let mut queue: VecDeque<i32> = {
            let mut queue = VecDeque::with_capacity(len_ns * 2);
            queue.push_back(nums[0]);
            queue.push_back(-nums[0]);
            queue
        };
        while !queue.is_empty() {
            if lvl == len_ns {
                break;
            }
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    let sum_plus: i32 = cur + nums[lvl];
                    queue.push_back(sum_plus);
                    let sum_minus: i32 = cur - nums[lvl];
                    queue.push_back(sum_minus);
                }
            }
            lvl += 1;
        }
        return queue.into_iter().filter(|&sum| sum == target).count() as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![1, 1, 1, 1, 1];
        let target: i32 = 3;
        let expected: i32 = 5;
        let actual: i32 = Solution::find_target_sum_ways(nums, target);
        assert_eq!(expected, actual);
    }
}
