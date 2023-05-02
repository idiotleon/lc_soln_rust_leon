/// @author: Leon
/// https://leetcode.com/problems/sign-of-the-product-of-an-array/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let mut cnt_n: u16 = 0;
        for num in nums {
            if num < 0 {
                cnt_n += 1;
            } else if num == 0 {
                return 0;
            }
        }
        return if cnt_n % 2 == 0 { 1 } else { -1 };
    }
}
