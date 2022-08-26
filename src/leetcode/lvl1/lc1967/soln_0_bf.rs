/// @author: Leon
/// https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word/
/// Time Complexity:    O(`len_ps` * `len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word/discuss/1404046/C%2B%2B-No-fancy-O(NM)-solution
/// https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word/discuss/1404073/Python3-1-line
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let len_ps: usize = patterns.len();
        let len_s: usize = word.len();
        let mut cnt: i32 = 0;
        for p in patterns{
            if word.contains(&p){
                cnt += 1;
            }
        }
        return cnt;
    }
}