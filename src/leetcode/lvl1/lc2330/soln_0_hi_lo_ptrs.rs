/// @author: Leon
/// https://leetcode.com/problems/valid-palindrome-iv/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn make_palindrome(s: String) -> bool {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut lo: usize = 0;
        let mut hi: usize = len_s - 1;
        let mut cnt: u8 = 0;
        while lo < hi{
            if chs[lo] != chs[hi]{
                cnt += 1;
                if cnt > 2{
                    return false;
                }
            }
            lo += 1;
            hi -= 1;
        }
        true
    }
}