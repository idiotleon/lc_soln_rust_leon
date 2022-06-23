/// @author: Leon
/// https://leetcode.com/problems/subarray-sum-equals-k/
/// Time Complexity:    O(`len_n` ^ 3)
/// Space Complexity:   O(1)
/// This approach leads to TLE.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut cnt: i32 = 0;
        for lo in 0..len_n {
            for hi in lo..len_n {
                if k == Self::get_sum(lo, hi, &nums) {
                    cnt += 1;
                };
            }
        }
        cnt
    }
    fn get_sum(lo: usize, hi: usize, nums: &Vec<i32>) -> i32 {
        let mut sum: i32 = 0;
        let mut idx = lo;
        while idx <= hi {
            sum += nums[idx];
            idx += 1;
        }
        sum
    }
}
