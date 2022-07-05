/// @author: Leon
/// https://leetcode.com/problems/maximum-average-subarray-ii/
/// Time Complexity:    O(`len_ns` * lg(`RANGE`))
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/maximum-average-subarray-ii/discuss/105480/Java-solution-O(nlogM)-Binary-search-the-answer/108071
/// https://leetcode.com/problems/maximum-average-subarray-ii/discuss/105480/Java-solution-O(nlogM)-Binary-search-the-answer/108071
/// https://leetcode.com/problems/maximum-average-subarray-ii/discuss/105480/Java-solution-O(nlogM)-Binary-search-the-answer/108070
/// https://leetcode.com/problems/maximum-average-subarray-ii/discuss/105480/Java-solution-O(nlogM)-Binary-search-the-answer
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        const RANGE: f64 = 1e4_f64;
        const TOLERANCE: f64 = 1e-5_f64;
        let mut lo: f64 = -RANGE;
        let mut hi: f64 = RANGE;
        while hi - lo > TOLERANCE {
            let mid = (lo + hi) / 2_f64;
            if Self::search(mid, &nums, k as usize) >= 0_f64 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (lo + hi) / 2_f64
    }
    fn search(guess: f64, nums: &Vec<i32>, len: usize) -> f64 {
        let len_ns: usize = nums.len();
        let mut sum: f64 = {
            let mut sum: f64 = 0_f64;
            for idx in 0..len {
                sum += nums[idx] as f64 - guess;
            }
            sum
        };
        let mut max_sum: f64 = sum;
        let mut prev: f64 = nums[0] as f64 - guess;
        for idx in len..len_ns {
            sum = sum - nums[idx - len] as f64 + nums[idx] as f64;
            max_sum = max_sum.max(sum.max(sum + prev));
            prev = (nums[idx - len + 1] as f64).max(nums[idx - len + 1] as f64 + prev) - guess;
        }
        max_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![1, 12, -5, -6, 50, 3];
        let k: i32 = 4;
        let expected: f64 = 12.75000_f64;
        let actual = Solution::find_max_average(nums, k);
        assert!((expected - actual).abs() < 1e-5_f64);
    }
}
