/// @author: Leon
/// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut ans: i32 = 0;
        let mut lo: usize = 0;
        let mut hi: usize = len_ns - 1;
        while lo < hi {
            ans = std::cmp::max(ans, sorted[lo] + sorted[hi]);
            lo += 1;
            hi -= 1;
        }
        return ans;
    }
}
