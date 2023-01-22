/// @author: Leon
/// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/
/// Time Complexity:    O(`_len_ns`)
/// Space Compleixty:   O(`RANGE`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        const RANGE: i32 = 1000 + 1;
        let _len_ns: usize = nums.len();
        let mut freqs: Vec<i32> = vec![0; RANGE as usize * 2];
        let mut largest: i32 = -1;
        for num in nums {
            let idx: usize = (num + RANGE) as usize;
            freqs[idx] += 1;
            let idx_neg: usize = (-num + RANGE) as usize;
            if freqs[idx_neg] > 0 && num.abs() > largest {
                largest = num.abs();
            }
        }
        return largest;
    }
}
