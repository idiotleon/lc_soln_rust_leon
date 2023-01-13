/// @author: Leon
/// https://leetcode.com/problems/di-string-match/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        const INC: char = 'I';
        const DEC: char = 'D';
        let len_s: usize = s.len();
        let mut ans: Vec<i32> = vec![0; len_s + 1];
        let mut lo: i32 = 0;
        let mut hi: i32 = len_s as i32;
        let mut idx: usize = 0;
        for ch in s.chars() {
            match ch {
                INC => {
                    ans[idx] = lo;
                    lo += 1;
                }
                DEC => {
                    ans[idx] = hi;
                    hi -= 1;
                }
                _ => unreachable!(),
            }
            idx += 1;
        }
        ans[len_s] = lo;
        return ans;
    }
}
