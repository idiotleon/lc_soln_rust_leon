/// @author: Leon
/// https://leetcode.com/problems/product-of-array-except-self/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len_n = nums.len();
        let ans = {
            let mut ans = vec![0; len_n];
            if nums.is_empty() {
                return ans;
            }
            ans[0] = 1;
            for i in 1..len_n {
                ans[i] = nums[i - 1] * ans[i - 1];
            }
            let mut res = 1;
            for i in (0..len_n).rev() {
                ans[i] *= res;
                res *= nums[i];
            }
            ans
        };
        ans
    }
}
