/// @author: Leon
/// https://leetcode.com/problems/arithmetic-subarrays/
/// Time Complexity:    O(`len_ms` * `len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let len_ns: usize = nums.len();
        let len_ms: usize = l.len();
        let mut ans: Vec<bool> = Vec::with_capacity(len_ns);
        for idx in 0..len_ms {
            let lo: usize = l[idx] as usize;
            let hi: usize = r[idx] as usize;
            let res = if Self::is_valid(lo, hi, &nums) {
                true
            } else {
                false
            };
            ans.push(res);
        }
        return ans;
    }
    fn is_valid(lo: usize, hi: usize, nums: &Vec<i32>) -> bool {
        let len_ss: usize = hi - lo + 1;
        if len_ss <= 1 {
            return true;
        }
        let sorted: Vec<i32> = {
            let mut seg = nums[lo..=hi].to_vec();
            seg.sort();
            seg
        };
        let expected: i32 = sorted[1] - sorted[0];
        for idx in 1..len_ss {
            let diff = sorted[idx] - sorted[idx - 1];
            if diff != expected {
                return false;
            }
        }
        return true;
    }
}
