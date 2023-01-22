/// @author: Leon
/// https://leetcode.com/problems/minimum-cost-to-split-an-array/
/// Time Complexity:    O(`len_ns` ^ 2)
/// Space Complexity:   O(`len_ns` ^ 2)
/// Reference:
/// https://leetcode.com/problems/minimum-cost-to-split-an-array/solutions/3083781/java-dp/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let trimmed: Vec<Vec<i32>> = {
            let mut trimmed: Vec<Vec<i32>> = vec![vec![0; len_ns]; len_ns];
            for lo in 0..len_ns {
                // to calculate the trimmed length !on the run!
                let mut cnt: i32 = 0;
                let mut freqs: Vec<i32> = vec![0; len_ns];
                for hi in lo..len_ns {
                    let idx: usize = nums[hi] as usize;
                    freqs[idx as usize] += 1;
                    if freqs[idx] == 2 {
                        cnt += 2;
                    } else if freqs[idx] > 2 {
                        cnt += 1;
                    }
                    trimmed[lo][hi] = cnt;
                }
            }
            trimmed
        };
        // double caution with indices
        let upper_bound: i32 = trimmed[0][len_ns - 1] + k;
        // double caution with indices
        let mut dp: Vec<i32> = vec![0; len_ns + 1];
        for hi in 1..=len_ns {
            let mut min: i32 = upper_bound;
            for lo in 0..hi {
                min = std::cmp::min(min, dp[lo] + k + trimmed[lo][hi - 1]);
            }
            dp[hi] = min;
        }
        return dp[len_ns];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 1, 3, 3];
        let k: i32 = 2;
        let actual = Solution::min_cost(nums, k);
        let expected: i32 = 8;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_2_should_return_expected() {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 1];
        let k: i32 = 2;
        let actual = Solution::min_cost(nums, k);
        let expected: i32 = 6;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_3_should_return_expected() {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 1];
        let k: i32 = 5;
        let actual = Solution::min_cost(nums, k);
        let expected: i32 = 10;
        assert_eq!(expected, actual);
    }
}
