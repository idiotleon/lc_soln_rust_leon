/// @author: Leon
/// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/discuss/252242/JavaC%2B%2BPython-Different-Ideas
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let len_n: usize = tops.len();
        const RANGE: usize = 6 + 1;
        let (freq_t, freq_b, freq_same): (Vec<u16>, Vec<u16>, Vec<u16>) = {
            let mut freq_t: Vec<u16> = vec![0; RANGE];
            let mut freq_b: Vec<u16> = vec![0; RANGE];
            let mut freq_same: Vec<u16> = vec![0; RANGE];
            for idx in 0..len_n {
                freq_t[tops[idx] as usize] += 1;
                freq_b[bottoms[idx] as usize] += 1;
                if tops[idx] == bottoms[idx] {
                    freq_same[tops[idx] as usize] += 1;
                }
            }
            (freq_t, freq_b, freq_same)
        };
        for idx in 1..RANGE {
            if freq_t[idx] + freq_b[idx] - freq_same[idx] == len_n as u16 {
                return len_n as i32 - std::cmp::max(freq_t[idx], freq_b[idx]) as i32;
            }
        }
        -1
    }
}
