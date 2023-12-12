/// @author: Leon
/// https://leetcode.com/problems/count-and-say/
/// Time Complexity:    O(4 ^ (`n` / 3))
/// Space Complexity:   O(4 ^ (`n` / 3)) / O(1)
/// Reference:
/// https://leetcode.com/problems/count-and-say/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let n: usize = n as usize;
        let mut cur: String = "1".to_owned();
        // to approach the problem in an iterative way
        for _idx in 2..=n {
            let len_s: usize = cur.len();
            let chs: Vec<char> = cur.chars().collect();
            let mut nxt: String = String::new();
            let mut lo: usize = 0;
            let mut hi: usize = 0;
            while lo < len_s {
                // to find the count of the same characters
                while hi < len_s && chs[lo] == chs[hi] {
                    hi += 1;
                }
                nxt = format!("{}{}{}", nxt, hi - lo, chs[lo]);
                // to have the `lo` jump to `hi`
                lo = hi;
            }
            cur = nxt;
        }
        return cur;
    }
}
