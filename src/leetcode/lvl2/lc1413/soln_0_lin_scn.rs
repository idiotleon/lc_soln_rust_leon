/// @author: Leon
/// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/discuss/1570965/C%2B%2BPython-Clean-and-Simple-Solution-w-Explanation-or-Tracking-min-Running-Sum
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let _len_n = nums.len();
        let ans = {
            let mut ans = 0;
            let mut sum = 0;
            for num in nums {
                sum += num;
                ans = std::cmp::min(ans, sum);
            }
            ans
        };
        -ans + 1
    }
}
