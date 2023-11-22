/// @author: Leon
/// https://leetcode.com/problems/target-sum/
/// Time Complexity:    O(2 ^ `_len_ns`)
/// Space Complexity:   O(`_len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let _len_ns: usize = nums.len();
        let mut cnt: i32 = 0;
        Self::dfs(0, 0, &nums, target, &mut cnt);
        return cnt;
    }
    fn dfs(idx: usize, sum_prev: i32, nums: &Vec<i32>, target: i32, cnt: &mut i32) {
        let len_ns: usize = nums.len();
        if idx >= len_ns {
            return;
        }
        let sum_plus = sum_prev + nums[idx];
        let sum_minus = sum_prev - nums[idx];
        if idx == len_ns - 1 {
            if sum_plus == target {
                *cnt += 1;
            }
            if sum_minus == target {
                *cnt += 1;
            }
            return;
        }
        Self::dfs(idx + 1, sum_plus, nums, target, cnt);
        Self::dfs(idx + 1, sum_minus, nums, target, cnt);
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
