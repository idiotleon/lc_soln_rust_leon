/// @author: Leon
/// https://leetcode.com/problems/apply-bitwise-operations-to-make-strings-equal/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/apply-bitwise-operations-to-make-strings-equal/solutions/3083831/java-c-python-1-line-check-1/?orderBy=hot
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        let _len_s: usize = s.len();
        return s.contains("1") == target.contains("1");
    }
}
