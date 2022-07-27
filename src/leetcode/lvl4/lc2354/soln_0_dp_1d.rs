use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/number-of-excellent-pairs/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/number-of-excellent-pairs/discuss/2324984/JavaC%2B%2BPython-Inclusion-Exclusion-Principle
/// Note:
/// the inclusion-exclusion principle:
/// bits(num1 OR num2) + bits(num1 AND num2) = bits(num1) + bits(num2)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let _len_ns: usize = nums.len();
        let set: HashSet<i32> = nums.into_iter().collect::<HashSet<i32>>();
        let freqs: Vec<i64> = {
            let mut freqs: Vec<i64> = vec![0; 30];
            for &num in set.iter() {
                freqs[num.count_ones() as usize] += 1;
            }
            freqs
        };
        let mut cnt: i64 = 0;
        for bit1 in 1..30 {
            for bit2 in 1..30 {
                if bit1 + bit2 >= k {
                    cnt += freqs[bit1 as usize] * freqs[bit2 as usize];
                }
            }
        }
        cnt
    }
}
