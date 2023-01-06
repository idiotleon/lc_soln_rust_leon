/// @author: Leon
/// https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array/
/// Time Complexity:    O(`len_ws` * avg_len_word)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, idx_start: i32) -> i32 {
        let len_ws: isize = words.len() as isize;
        let mut is_met: bool = false;
        let mut cnt: i32 = 0;
        let mut idx_lo: isize = idx_start as isize;
        let mut idx_hi: isize = idx_start as isize;
        while !is_met {
            if &words[Self::get_idx_lo(idx_lo, len_ws)] == &target
                || &words[Self::get_idx_hi(idx_hi, len_ws)] == &target
            {
                return cnt;
            }
            cnt += 1;
            idx_lo -= 1;
            idx_hi += 1;
            if Self::get_idx_lo(idx_lo, len_ws) == Self::get_idx_hi(idx_hi, len_ws) {
                if &words[Self::get_idx_lo(idx_lo, len_ws)] == &target {
                    return cnt;
                }
                is_met = true;
            }
        }
        return -1;
    }
    fn get_idx_lo(idx: isize, len_ws: isize) -> usize {
        return ((idx + len_ws) % len_ws) as usize;
    }
    fn get_idx_hi(idx: isize, len_ws: isize) -> usize {
        return (idx % len_ws) as usize;
    }
}
