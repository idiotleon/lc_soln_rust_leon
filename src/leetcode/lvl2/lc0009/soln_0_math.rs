/// @author: Leon
/// https://leetcode.com/problems/palindrome-number/
/// Time Complexity:    O(L)
/// Space Complexity:   O(1) / O(L)
/// Reference:
/// https://leetcode.com/problems/palindrome-number/discuss/5127/9-line-accepted-Java-code-without-the-need-of-handling-overflow
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut rev: i32 = 0;
        let mut x: i32 = x;
        while x != 0 {
            let digit = x % 10;
            let new_res = rev * 10 + digit;
            if (new_res - digit) / 10 != rev {
                return 0;
            }
            rev = new_res;
            x /= 10;
        }
        rev
    }
}
