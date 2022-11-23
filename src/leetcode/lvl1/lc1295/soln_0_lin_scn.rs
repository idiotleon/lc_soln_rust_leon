/// @author: Leon
/// https://leetcode.com/problems/find-numbers-with-even-number-of-digits/description/
/// Time Complexity:    O(`len_ns` * avg_len_num)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let mut cnt: u16 = 0;
        for num in nums {
            let len = Self::get_len(num);
            if len % 2 == 0 {
                cnt += 1;
            }
        }
        return cnt as i32;
    }
    fn get_len(num: i32) -> u16 {
        let mut src = num;
        let mut cnt: u16 = 0;
        while src > 0 {
            src /= 10;
            cnt += 1;
        }
        return cnt;
    }
}
