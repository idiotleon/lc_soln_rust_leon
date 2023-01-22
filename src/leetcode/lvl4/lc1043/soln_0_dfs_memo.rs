/// @author: Leon
/// https://leetcode.com/problems/partition-array-for-maximum-sum/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/partition-array-for-maximum-sum/discuss/370807/dfs-solution-using-memoization-super-easy-to-understand
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sum_after_partitioning(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let mut memo: Vec<Option<i32>> = vec![None; len_ns];
        Self::dfs(0, k as usize, &nums, &mut memo)
    }
    fn dfs(idx_start: usize, k: usize, nums: &Vec<i32>, memo: &mut Vec<Option<i32>>) -> i32 {
        let len_ns: usize = nums.len();
        if idx_start >= len_ns {
            return 0;
        }
        if let Some(m) = memo[idx_start] {
            return m;
        }
        let mut cur_max: i32 = 0;
        let mut max_sum: i32 = 0;
        for idx in idx_start..std::cmp::min(len_ns, idx_start + k) {
            cur_max = std::cmp::max(cur_max, nums[idx]);
            max_sum = std::cmp::max(
                max_sum,
                cur_max * (idx - idx_start + 1) as i32 + Self::dfs(idx + 1, k, &nums, memo),
            );
        }
        memo[idx_start] = Some(max_sum);
        return max_sum;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![1, 15, 7, 9, 2, 5, 10];
        let k: i32 = 3;
        let actual = Solution::max_sum_after_partitioning(nums, k);
        let expected: i32 = 84;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_2_should_return_expected() {
        let nums: Vec<i32> = vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3];
        let k: i32 = 4;
        let actual = Solution::max_sum_after_partitioning(nums, k);
        let expected: i32 = 83;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_3_should_return_expected() {
        let nums: Vec<i32> = vec![1];
        let k: i32 = 1;
        let actual = Solution::max_sum_after_partitioning(nums, k);
        let expected: i32 = 1;
        assert_eq!(expected, actual);
    }
}
