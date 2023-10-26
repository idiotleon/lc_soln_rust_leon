/// @author: Leon
/// https://leetcode.com/problems/string-compression/
/// Time Complexity:    O(`len_cs`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn compress(chs: &mut Vec<char>) -> i32 {
        let len_cs: usize = chs.len();
        let mut idx_res: usize = 0;
        let mut idx: usize = 0;
        while idx < len_cs {
            let len: usize = {
                let mut len: usize = 0;
                while idx + len < len_cs && chs[idx + len] == chs[idx] {
                    len += 1;
                }
                len
            };
            chs[idx_res] = chs[idx];
            idx_res += 1;
            if len > 1 {
                for ch in len.to_string().chars().collect::<Vec<char>>() {
                    chs[idx_res] = ch;
                    idx_res += 1;
                }
            }
            idx += len;
        }
        return idx_res as i32;
    }
}
