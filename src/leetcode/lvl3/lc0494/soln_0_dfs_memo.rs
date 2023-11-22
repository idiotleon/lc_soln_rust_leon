/// @author: Leon
/// https://leetcode.com/problems/target-sum/
/// Time Complexity:    O(`len_ns` * `sum`)
/// Space Complexity:   O(`len_ns` * `sum`)
/// Reference:
/// https://leetcode.com/problems/target-sum/discuss/97333/Java-simple-DFS-with-memorization/356995
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let len_ns: usize = nums.len();
        let sum: i32 = nums.iter().sum();
        if target > sum || target < -sum {
            return 0;
        }
        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; sum as usize * 2 + 1]; len_ns + 1];
        return Self::dfs(0, sum, target + sum, &nums, &mut memo);
    }
    fn dfs(
        idx: usize,
        sum: i32,
        target: i32,
        nums: &Vec<i32>,
        memo: &mut Vec<Vec<Option<i32>>>,
    ) -> i32 {
        let len_ns: usize = nums.len();
        if idx == len_ns {
            return if sum == target { 1 } else { 0 };
        }
        if let Some(m) = memo[idx][sum as usize] {
            return m;
        }
        let res = Self::dfs(idx + 1, sum + nums[idx], target, nums, memo)
            + Self::dfs(idx + 1, sum - nums[idx], target, nums, memo);
        memo[idx][sum as usize] = Some(res);
        return res;
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
    #[test]
    fn it_works_with_sample_input_2() {
        let nums: Vec<i32> = vec![1];
        let target: i32 = 1;
        let expected: i32 = 1;
        let actual: i32 = Solution::find_target_sum_ways(nums, target);
        assert_eq!(expected, actual);
    }
}
