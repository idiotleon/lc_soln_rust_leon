/// @author: Leon
/// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/
/// Time Complexity:    O(`_len_cds`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/discuss/2039717/Check-Each-Bit
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let _len_cds: usize = candidates.len();
        let mut longest: i32 = 0;
        let largest_num: i32 = *candidates.iter().max().unwrap();
        let mut cur_num: i32 = 1;
        while cur_num <= largest_num {
            let mut cnt: i32 = 0;
            for &candidate in &candidates {
                if candidate & cur_num > 0 {
                    cnt += 1;
                }
                longest = std::cmp::max(longest, cnt);
            }
            cur_num = cur_num << 1;
        }
        longest
    }
}
