/// @author: Leon
/// https://leetcode.com/problems/count-substrings-with-only-one-distinct-letter/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_s`) / O(1)
/// Reference:
/// https://leetcode.com/problems/count-substrings-with-only-one-distinct-letter/discuss/376921/Java-Simple-O(n)-code-brief-explanation-and-analysis.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_letters(s: String) -> i32 {
        let _len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        // the running length
        let mut len: usize = 1;
        let mut ans: i32 = 0;
        for (idx, &ch) in chs.iter().enumerate() {
            if idx > 0 {
                if ch == chs[idx - 1] {
                    len += 1;
                } else {
                    ans += Self::get_count(len as i32);
                    len = 1;
                }
            }
        }
        // note:
        // do not forget the last checkout
        ans + Self::get_count(len as i32)
    }
    /// to mathmetically calculate the count of substrings with all the same letters
    fn get_count(len: i32) -> i32 {
        len * (len + 1) / 2
    }
}
