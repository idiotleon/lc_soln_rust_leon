/// @author: Leon
/// https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let digits = Self::get_digits(num);
        digits[0] * 10 + digits[2] + digits[1] * 10 + digits[3]
    }
    fn get_digits(mut num: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity(4);
        while num > 0 {
            ans.push(num % 10);
            num /= 10;
        }
        ans.sort();
        ans
    }
}
