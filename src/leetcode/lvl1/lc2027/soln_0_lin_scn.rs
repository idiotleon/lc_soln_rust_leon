/// @author: Leon
/// https://leetcode.com/problems/minimum-moves-to-convert-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        const X: char = 'X';
        const O: char = 'O';
        let len_s: usize = s.len();
        let mut chs: Vec<char> = s.chars().collect();
        let mut cnt: i32 = 0;
        for idx in 0..len_s {
            if chs[idx] == X {
                cnt += 1;
                if idx + 1 < len_s {
                    chs[idx + 1] = O;
                }
                if idx + 2 < len_s {
                    chs[idx + 2] = O;
                }
            }
        }
        return cnt;
    }
}
