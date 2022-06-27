/// @author: Leon
/// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_partitions(s: String) -> i32 {
        let _len_s: usize = s.len();
        let freqs: Vec<u16> = {
            let mut freqs: Vec<u16> = vec![0; 10];
            for &b in s.as_bytes() {
                freqs[(b - b'0') as usize] += 1;
            }
            freqs
        };
        for (idx, freq) in freqs.into_iter().enumerate().rev() {
            if freq > 0 {
                return idx as i32;
            }
        }
        unreachable!()
    }
}
