/// @author: Leon
/// https://leetcode.com/problems/two-sum-less-than-k/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut lo: usize = 0;
        let mut hi: usize = len_ns - 1;
        let mut ans: i32 = -1;
        while lo < hi {
            let sum = sorted[lo] + sorted[hi];
            if sum >= k {
                hi -= 1;
            } else {
                ans = std::cmp::max(ans, sum);
                lo += 1;
            }
        }
        return ans;
    }
}
