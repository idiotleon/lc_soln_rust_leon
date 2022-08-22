/// @author: Leon
/// https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        const B: char = 'B';
        let len_s: usize = blocks.len();
        let chs: Vec<char> = blocks.chars().collect();
        let k: usize = k as usize;
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        let mut cnt: usize = 0;
        let mut most: usize = 0;
        while hi < len_s {
            if chs[hi] == B {
                cnt += 1;
            }
            most = std::cmp::max(most, cnt);
            if most >= k {
                return 0;
            }
            if hi - lo + 1 == k {
                if chs[lo] == B {
                    cnt -= 1;
                }
                lo += 1;
            }
            hi += 1;
        }
        return (k - most) as i32;
    }
}
