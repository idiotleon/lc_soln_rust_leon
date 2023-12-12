/// @author: Leon
/// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let mut max: i32 = 0;
        let mut max_sec: i32 = 0;
        for num in nums {
            if num > max {
                max_sec = max;
                max = num;
            } else if num > max_sec {
                max_sec = num;
            }
        }
        return (max - 1) * (max_sec - 1);
    }
}
