/// @author: Leon
/// https://leetcode.com/problems/maximum-average-subarray-i/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Note:
/// as of now, `f64` does not implement `Ord` trait, because of the existence of `NaN`.
/// https://users.rust-lang.org/t/maximum-of-two-floating-points-numbers/70302/2?u=leon
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        const RANGE: f64 = 1e4 as f64 + 7.0;
        let len_n: usize = nums.len();
        let len_k = k as usize;
        // the denominator
        let avg_k = k as f64;
        let mut sum: f64 = 0.0;
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        let mut largest: f64 = -RANGE;
        while hi < len_n {
            sum += nums[hi] as f64;
            if hi + 1 >= len_k {
                let avg: f64 = sum / avg_k;
                // as of now, `f64` does not implement `Ord` trait, because of the existence of `NaN`.
                // https://users.rust-lang.org/t/maximum-of-two-floating-points-numbers/70302/2?u=leon
                largest = largest.max(avg);
                // if avg > largest {
                //     largest = avg;
                // }
                sum -= nums[lo] as f64;
                lo += 1;
            }
            hi += 1;
        }
        largest
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_sample_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![1, 12, -5, -6, 50, 3];
        let k: i32 = 4;
        let actual = Solution::find_max_average(nums, k);
        let expected: f64 = 12.75;
        assert_eq!(expected, actual);
    }
}
