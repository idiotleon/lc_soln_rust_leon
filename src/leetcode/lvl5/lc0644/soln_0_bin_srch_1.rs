/// @author: Leon
/// https://leetcode.com/problems/maximum-average-subarray-ii/
/// Time Complexity:    O(`len_ns` * lg(`RANGE`))
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/maximum-average-subarray-ii/discuss/105480/Java-solution-O(nlogM)-Binary-search-the-answer/108070
/// https://leetcode.com/problems/maximum-average-subarray-ii/discuss/105480/Java-solution-O(nlogM)-Binary-search-the-answer
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RANGE: f64 = 1e4 as f64 + 7_f64;
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut lo: f64 = -Self::RANGE;
        let mut hi: f64 = Self::RANGE;
        for _ in 0..100 {
            let mid = (lo + hi) / 2_f64;
            if Self::search(mid, &nums, k as usize) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo
    }
    fn search(guess: f64, nums: &Vec<i32>, len: usize) -> bool {
        let len_ns: usize = nums.len();
        let cum: Vec<f64> = {
            let mut cum: Vec<f64> = vec![0_f64; len_ns + 1];
            for (idx, &num) in nums.iter().enumerate() {
                cum[idx + 1] = cum[idx] + num as f64 - guess;
            }
            cum
        };
        let mut min: f64 = Self::RANGE;
        for idx in len - 1..len_ns {
            min = min.min(cum[idx - (len - 1)]);
            if cum[idx + 1] - min >= 0_f64 {
                return true;
            };
        }
        false
    }
}
