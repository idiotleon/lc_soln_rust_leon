/// @author: Leon
/// https://leetcode.com/problems/count-the-digits-that-divide-a-number/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut src = num;
        let mut cnt: i32 = 0;
        while src > 0 {
            let digit = src % 10;
            if num % digit == 0 {
                cnt += 1;
            }
            src /= 10;
        }
        return cnt;
    }
}
