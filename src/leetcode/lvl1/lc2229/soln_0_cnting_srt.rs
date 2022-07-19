/// @author: Leon
/// https://leetcode.com/problems/check-if-an-array-is-consecutive/
/// Time Complexity:    O(`max - min`) + O(`len_ns`) ~ O(max(`max - min`, `len_ns`))
/// Space Complexity:   O(`max - min`) + O(`len_ns`) ~ O(max(`max - min`, `len_ns`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_consecutive(nums: Vec<i32>) -> bool {
        let len_ns: usize = nums.len();
        let min: i32 = *nums.iter().min().unwrap();
        let max: i32 = *nums.iter().max().unwrap();
        let freqs: Vec<u32> = {
            let range: usize = (max - min + 1) as usize;
            let mut freqs: Vec<u32> = vec![0; std::cmp::max(len_ns, range)];
            for num in nums {
                freqs[(num - min) as usize] += 1;
            }
            freqs
        };
        for freq in freqs {
            if freq == 0 {
                return false;
            }
        }
        true
    }
}
