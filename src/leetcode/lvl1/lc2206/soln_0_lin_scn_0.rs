/// @author: Leon
/// https://leetcode.com/problems/divide-array-into-equal-pairs/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`RANGE`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let _len_n: usize = nums.len();
        const RANGE: usize = 500 + 1;
        let freqs: Vec<u16> = {
            let mut freqs: Vec<u16> = vec![0; RANGE];
            for num in nums{
                freqs[num as usize] += 1;
            }
            freqs
        };
        for freq in freqs{
            if freq % 2 == 1{
                return false;
            }
        }
        true
    }
}