/// @author: Leon
/// https://leetcode.com/problems/keep-multiplying-found-values-by-two/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let _len_n: usize = nums.len();
        const RANGE: u16 = 1000 + 1;
        let exists: Vec<bool> = {
            let mut exists: Vec<bool> = vec![false; RANGE as usize];
            for num in nums {
                exists[num as usize] = true;
            }
            exists
        };
        let mut num: u16 = original as u16;
        while num < RANGE && exists[num as usize] {
            num *= 2
        }
        num as i32
    }
}
