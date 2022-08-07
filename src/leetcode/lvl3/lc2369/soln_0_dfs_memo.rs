/// @author: Leon
/// https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let len_ns: usize = nums.len();
        let mut memo: Vec<Option<bool>> = vec![None; len_ns];
        return Self::dfs(0, &nums, &mut memo);
    }
    fn dfs(idx: usize, nums: &Vec<i32>, memo: &mut Vec<Option<bool>>) -> bool {
        let len_ns: usize = nums.len();
        if idx > len_ns {
            return false;
        }
        if idx == len_ns {
            return true;
        }
        if let Some(m) = memo[idx] {
            return m;
        }
        let mut res: bool = false;
        if idx < len_ns - 1 {
            res = res || (nums[idx] == nums[idx + 1] && Self::dfs(idx + 2, nums, memo));
        }
        if idx < len_ns - 2 {
            res = res
                || (((nums[idx] == nums[idx + 1] && nums[idx + 1] == nums[idx + 2])
                    || (nums[idx + 2] - nums[idx + 1] == 1 && nums[idx + 1] - nums[idx] == 1))
                    && Self::dfs(idx + 3, nums, memo));
        }
        memo[idx] = Some(res);
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![4, 4, 4, 5, 6];
        let expected: bool = true;
        let actual: bool = Solution::valid_partition(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let nums: Vec<i32> = vec![1, 1, 1, 2];
        let expected: bool = false;
        let actual: bool = Solution::valid_partition(nums);
        assert_eq!(expected, actual);
    }
}
