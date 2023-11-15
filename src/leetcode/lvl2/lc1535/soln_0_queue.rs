use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/find-the-winner-of-an-array-game/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_winner(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let max: i32 = *nums.iter().max().unwrap();
        if k as usize >= len_ns {
            return max;
        }
        if k == 1 {
            return std::cmp::max(nums[0], nums[1]);
        }
        let mut queue: VecDeque<i32> = {
            let mut queue: VecDeque<i32> = VecDeque::with_capacity(len_ns);
            for num in nums {
                queue.push_back(num);
            }
            queue
        };
        let mut cnt: i32 = 1;
        let mut cur_max: i32 = 0;
        while let Some(first) = queue.pop_front() {
            if let Some(second) = queue.pop_front() {
                let max = std::cmp::max(first, second);
                if max == cur_max {
                    cnt += 1;
                    if cnt == k {
                        return max;
                    }
                } else if max > cur_max {
                    cnt = 1;
                    cur_max = max;
                }
                if first > second {
                    queue.push_front(first);
                    queue.push_back(second);
                } else {
                    queue.push_back(first);
                    queue.push_front(second);
                }
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![2, 1, 3, 5, 4, 6, 7];
        let k: i32 = 2;
        let expected: i32 = 5;
        let actual: i32 = Solution::get_winner(nums, k);
        assert_eq!(expected, actual);
    }
}
