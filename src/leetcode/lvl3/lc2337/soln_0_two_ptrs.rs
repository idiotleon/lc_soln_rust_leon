/// @author: Leon
/// https://leetcode.com/problems/move-pieces-to-obtain-a-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/move-pieces-to-obtain-a-string/discuss/2261229/Easy-or-O(N)-or-(Java-and-C%2B%2B)
/// https://leetcode.com/problems/move-pieces-to-obtain-a-string/discuss/2261392/JavaPython-3-O(n)-code%3A-compare-the-positions-of-'L'-and-'R'.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let len_s: usize = start.len();
        let len_t: usize = target.len();
        const SPACE: char = '_';
        const LEFT: char = 'L';
        const RIGHT: char = 'R';
        let chs_s: Vec<char> = start.chars().collect();
        let chs_t: Vec<char> = target.chars().collect();
        let mut idx_s: usize = 0;
        let mut idx_t: usize = 0;
        while idx_s < len_s || idx_t < len_t {
            while idx_s < len_s && chs_s[idx_s] == SPACE {
                idx_s += 1;
            }
            while idx_t < len_t && chs_t[idx_t] == SPACE {
                idx_t += 1;
            }
            if idx_s == len_s || idx_t == len_t {
                return idx_s == len_s && idx_t == len_t;
            }
            if chs_t[idx_t] != chs_s[idx_s] {
                return false;
            }
            if chs_t[idx_t] == LEFT {
                if idx_s < idx_t {
                    return false;
                }
            } else {
                if idx_s > idx_t {
                    return false;
                }
            }
            idx_s += 1;
            idx_t += 1;
        }
        true
    }
}
