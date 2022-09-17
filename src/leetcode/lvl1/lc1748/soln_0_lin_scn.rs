/// @author: Leon
/// https://leetcode.com/problems/sum-of-unique-elements/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        const RANGE: usize = 100 + 1;
        let _len_ns: usize = nums.len();
        let freqs: Vec<u8> = {
            let mut freqs: Vec<u8> = vec![0; RANGE];
            for num in nums {
                freqs[num as usize] += 1;
            }
            freqs
        };
        let mut sum: i32 = 0;
        for (num, freq) in freqs.into_iter().enumerate() {
            if freq == 1 {
                sum += num as i32;
            }
        }
        return sum;
    }
}
