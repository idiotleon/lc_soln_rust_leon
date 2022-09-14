/// @author: Leon
/// https://leetcode.com/problems/optimal-partition-of-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut freqs: Vec<u16> = vec![0; 26];
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        let mut cnt: i32 = 0;
        while hi < len_s {
            while hi < len_s && freqs[chs[hi] as usize - 'a' as usize] == 0 {
                freqs[chs[hi] as usize - 'a' as usize] += 1;
                hi += 1;
            }
            cnt += 1;
            while lo < hi {
                freqs[chs[lo] as usize - 'a' as usize] = 0;
                lo += 1;
            }
        }
        return cnt;
    }
}
