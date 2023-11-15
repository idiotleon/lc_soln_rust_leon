/// @author: Leon
/// https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut res = nums;
            res.sort();
            res
        };
        let mut ans: i32 = 1;
        let mut upper_bound: i32 = 1;
        for idx in 0..len_ns {
            if sorted[idx] < upper_bound {
                upper_bound = sorted[idx];
            }
            ans = std::cmp::max(ans, std::cmp::min(upper_bound, sorted[idx]));
            upper_bound += 1;
        }
        return ans;
    }
}
