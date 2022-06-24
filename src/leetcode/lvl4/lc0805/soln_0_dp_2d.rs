/// @author: Leon
/// https://leetcode.com/problems/split-array-with-same-average/
/// Time Complexity:    O((`len_n` ^ 2) * `sum_total`)
/// Space Complexity:   O(`len_n` * `sum_total`)
/// Reference:
/// https://leetcode.com/problems/split-array-with-same-average/discuss/120667/C%2B%2B-Solution-with-explanation-early-termination-(Updated-for-new-test-case)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let len_n: usize = nums.len();
        let sum_total: usize = nums.iter().sum::<i32>() as usize;
        // optimization
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut dp: Vec<Vec<bool>> = {
            let mut dp: Vec<Vec<bool>> = vec![vec![false; len_n / 2 + 1]; sum_total + 1];
            dp[0][0] = true;
            dp
        };
        for num in sorted {
            let num: usize = num as usize;
            for sum in (num..=sum_total).rev() {
                for idx in 1..=len_n / 2 {
                    dp[sum][idx] = dp[sum][idx] || dp[sum - num][idx - 1];
                }
            }
        }
        for idx in 1..=len_n / 2 {
            if sum_total * idx % len_n == 0 && dp[sum_total * idx / len_n][idx] {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_test_case_110_should_return_expected() {
        let nums: Vec<i32> = vec![0, 0, 0, 0, 0];
        let actual = Solution::split_array_same_average(nums);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
