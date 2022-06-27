/// @author: Leon
/// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_partitions(s: String) -> i32 {
        let _len_s: usize = s.len();
        let mut largest: u8 = 0;
        for &b in s.as_bytes() {
            let cur: u8 = b - b'0';
            if cur > largest {
                largest = cur;
            }
            // slight optimization
            if cur == b'9' {
                return 9;
            }
        }
        largest as i32
    }
}
