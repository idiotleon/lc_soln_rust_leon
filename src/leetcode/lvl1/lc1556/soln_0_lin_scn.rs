/// @author: Leon
/// https://leetcode.com/problems/thousand-separator/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1) / O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        if n == 0 {
            return "0".to_owned();
        }
        const DOT: char = '.';
        let mut src = n;
        let mut ans: String = "".to_owned();
        let mut cnt: u8 = 0;
        while src > 0 {
            let digit: u8 = (src % 10) as u8;
            ans.push((digit + '0' as u8) as char);
            cnt += 1;
            if src > 10 && cnt % 3 == 0 {
                ans.push(DOT);
                cnt = 0;
            }
            src /= 10;
        }
        return ans.chars().rev().collect::<String>();
    }
}
