/// @author: Leon
/// https://leetcode.com/problems/frequency-of-the-most-frequent-element/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/frequency-of-the-most-frequent-element/editorial/?envType=daily-question&envId=2023-11-18
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let prefix_sums: Vec<i64> = {
            let mut prefix_sums: Vec<i64> = vec![0; len_ns];
            prefix_sums[0] = sorted[0] as i64;
            for idx in 1..len_ns {
                prefix_sums[idx] = sorted[idx] as i64 + prefix_sums[idx - 1];
            }
            prefix_sums
        };
        let mut ans: i32 = 0;
        for idx in 0..len_ns {
            ans = std::cmp::max(
                ans,
                Self::check(idx as isize, k as i64, &sorted, &prefix_sums),
            );
        }
        return ans;
    }
    fn check(idx: isize, k: i64, nums: &Vec<i32>, prefix_sums: &Vec<i64>) -> i32 {
        let target: i64 = nums[idx as usize] as i64;
        let mut lo: isize = 0;
        let mut hi: isize = idx;
        let mut best: isize = idx;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let cnt = idx - mid + 1;
            let sum_exp = cnt as i64 * target;
            let sum_act =
                prefix_sums[idx as usize] - prefix_sums[mid as usize] + nums[mid as usize] as i64;
            let needed = sum_exp - sum_act;
            if needed > k {
                lo = mid + 1;
            } else {
                best = mid;
                hi = mid - 1;
            }
        }
        return (idx - best + 1) as i32;
    }
}
