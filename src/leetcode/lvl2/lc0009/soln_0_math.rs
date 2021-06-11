/// https://leetcode.com/problems/palindrome-number/
/// 
/// Time Complexity:    O(L)
/// Space Complexity:   O(1) / O(L)
/// 
/// Reference:
/// https://leetcode.com/problems/palindrome-number/discuss/5127/9-line-accepted-Java-code-without-the-need-of-handling-overflow
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x != 0 && x % 10 == 0){
            return false;
        }
        
        let mut x = x;
        let mut rev = 0;
        
        while x > rev{
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        
        x == rev || x == (rev / 10)
    }
}