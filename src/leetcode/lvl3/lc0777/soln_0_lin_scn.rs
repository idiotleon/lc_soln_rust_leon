/// @author: Leon
/// https://leetcode.com/problems/swap-adjacent-in-lr-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/swap-adjacent-in-lr-string/discuss/113789/Simple-Java-one-pass-O(n)-solution-with-explaination
/// https://leetcode.com/problems/swap-adjacent-in-lr-string/discuss/113789/Simple-Java-one-pass-O(n)-solution-with-explaination
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        const X: char = 'X';
        const L: char = 'L';
        const R: char = 'R';
        let len_s: usize = start.len();
        let len_e: usize = end.len();
        let chs_s: Vec<char> = start.chars().collect();
        let chs_e: Vec<char> = end.chars().collect();
        let mut idx_s: usize = 0;
        let mut idx_e: usize = 0;
        while idx_s < len_s || idx_e < len_e {
            while idx_s < len_s && chs_s[idx_s] == X {
                idx_s += 1;
            }
            while idx_e < len_e && chs_e[idx_e] == X {
                idx_e += 1;
            }
            if idx_s == len_s && idx_e == len_e {
                return true;
            }
            if idx_s == len_s || idx_e == len_e {
                return false;
            }
            if chs_s[idx_s] != chs_e[idx_e] {
                return false;
            }
            if chs_s[idx_s] == L && idx_s < idx_e {
                return false;
            }
            if chs_s[idx_s] == R && idx_s > idx_e {
                return false;
            }
            idx_s += 1;
            idx_e += 1;
        }
        idx_s == idx_e
    }
}
