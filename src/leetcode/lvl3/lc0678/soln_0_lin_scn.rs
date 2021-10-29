/// https://leetcode.com/problems/valid-parenthesis-string/
///
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_s`) / O(1)
///
/// Reference:
/// https://leetcode.com/problems/valid-parenthesis-string/discuss/302732/C%2B%2B-O(S)-Time-O(1)-Space-One-Pass-with-Explanation
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let _len_s: usize = s.len();
        let mut max_diff: i32 = 0;
        let mut min_diff: i32 = 0;
        for ch in s.chars() {
            max_diff += if ch == '(' || ch == '*' { 1 } else { -1 };
            min_diff += if ch == ')' || ch == '*' { -1 } else { 1 };
            if max_diff < 0 {
                return false;
            }
            min_diff = std::cmp::max(0, min_diff);
        }
        min_diff == 0
    }
}
