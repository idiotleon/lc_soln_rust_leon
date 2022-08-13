/// @author: Leon
/// https://leetcode.com/problems/finding-3-digit-even-numbers/
/// Time Complexity:    O(`_len_ds`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/finding-3-digit-even-numbers/discuss/1612150/Three-Cycles
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let _len_ds: usize = digits.len();
        let freqs: Vec<u8> = {
            let mut freqs: Vec<u8> = vec![0; 10];
            for digit in digits {
                freqs[digit as usize] += 1;
            }
            freqs
        };
        let mut ans: Vec<i32> = Vec::new();
        for lo in 1..10 {
            if freqs[lo] == 0 {
                continue;
            }
            for mid in 0..10 {
                if freqs[mid] > if lo == mid { 1 } else { 0 } {
                    for hi in (0..10).step_by(2) {
                        if freqs[hi] > if hi == lo { 1 } else { 0 } + if mid == hi { 1 } else { 0 }
                        {
                            ans.push((lo * 100 + mid * 10 + hi) as i32);
                        }
                    }
                }
            }
        }
        return ans;
    }
}
