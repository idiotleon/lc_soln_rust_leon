/// @author: Leon
/// https://leetcode.com/problems/count-binary-substrings/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/count-binary-substrings/discuss/108625/JavaC%2B%2BPython-Easy-and-Concise-with-Explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        // the previous running length
        let mut prev: i32 = 0;
        // the current running length
        let mut cur: i32 = 1;
        let mut cnt: i32 = 0;
        for idx in 1..len_s {
            if chs[idx - 1] == chs[idx] {
                cur += 1;
            } else {
                cnt += std::cmp::min(prev, cur);
                prev = cur;
                cur = 1;
            }
        }
        cnt + std::cmp::min(prev, cur)
    }
}
