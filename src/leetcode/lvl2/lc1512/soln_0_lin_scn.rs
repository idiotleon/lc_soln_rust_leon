/// @author: Leon
/// https://leetcode.com/problems/number-of-good-pairs/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`RANGE`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        const RANGE: usize = 100 + 1;
        let _len_n: usize = nums.len();
        let mut freqs: Vec<u8> = vec![0; RANGE];
        let mut ans: u16 = 0;
        for num in nums {
            ans += freqs[num as usize] as u16;
            freqs[num as usize] += 1;
        }
        ans as i32
    }
}
