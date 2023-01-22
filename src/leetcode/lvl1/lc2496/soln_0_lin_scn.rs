/// @author: Leon
/// https://leetcode.com/problems/maximum-value-of-a-string-in-an-array/
/// Time Complexity:    O(`_len_ws` * avg_len_word)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        let _len_ws: usize = strs.len();
        return strs.into_iter()
            .map(|wd| Self::get_value(wd.chars().collect()))
            .max()
            .unwrap_or(-1);
    }
    fn get_value(chs: Vec<char>) -> i32 {
        let len_cs: usize = chs.len();
        let mut ans: i32 = 0;
        for ch in chs {
            if ch >= '0' && ch <= '9' {
                ans *= 10;
                ans += ch as i32 - '0' as i32;
            } else {
                return len_cs as i32;
            }
        }
        return ans;
    }
}
