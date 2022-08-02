/// @author: Leon
/// https://leetcode.com/problems/most-frequent-number-following-key-in-an-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`RANGE`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let len_ns: usize = nums.len();
        const RANGE: usize = 1e3 as usize + 1;
        let freqs: Vec<u16> = {
            let mut freqs: Vec<u16> = vec![0; RANGE];
            for idx in 0..len_ns - 1 {
                if nums[idx] == key {
                    freqs[nums[1 + idx] as usize] += 1;
                }
            }
            freqs
        };
        let mut ans: usize = 0;
        let mut max: u16 = 0;
        for (idx, freq) in freqs.into_iter().enumerate() {
            if freq > max {
                ans = idx;
                max = freq;
            }
        }
        ans as i32
    }
}
