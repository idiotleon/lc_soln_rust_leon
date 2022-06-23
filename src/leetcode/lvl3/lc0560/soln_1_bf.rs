/// @author: Leon
/// https://leetcode.com/problems/subarray-sum-equals-k/
/// Time Complexity:    O(`len_n` ^ 2)
/// Space Complexity:   O(1)
/// This approach barely passes.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut cnt: i32 = 0;
        for lo in 0..len_n {
            let mut sum: i32 = 0;
            for hi in lo..len_n {
                sum += nums[hi];
                if k == sum{
                    cnt += 1;
                }
            }
            sum -= nums[lo];
        }
        cnt
    }
}
