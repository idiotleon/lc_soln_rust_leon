/// @author: Leon
/// https://leetcode.com/problems/sort-array-by-increasing-frequency/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(1) / O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let _len_n: usize = nums.len();
        const RANGE: i32 = 100 + 1;
        let freqs: Vec<u8> = {
            let mut freqs: Vec<u8> = vec![0; RANGE as usize];
            for &num in &nums {
                freqs[num as usize] += 1;
            }
            freqs
        };
        let sorted: Vec<i32> = {
            let mut sorted = nums;
            sorted.sort_by(|&a, &b| {
                if freqs[a as usize] != freqs[b as usize] {
                    freqs[a as usize].cmp(&freqs[b as usize])
                } else {
                    b.cmp(&a)
                }
            });
            sorted
        };
        sorted
    }
}
