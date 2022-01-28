/// @author: Leon
/// https://leetcode.com/problems/maximum-product-of-three-numbers/
/// Time Complexity:    O(`len_n` * lg(`len_n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        nums.sort();
        let product1: i32 = nums[0] * nums[1] * nums[len_n - 1];
        let product2: i32 = nums[len_n - 3] * nums[len_n - 2] * nums[len_n - 1];
        std::cmp::max(product1, product2)
    }
}
