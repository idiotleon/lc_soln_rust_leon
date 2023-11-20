/// @author: Leon
/// https://leetcode.com/problems/separate-the-digits-in-an-array/
/// Time Complexity:    O(`RANGE`) ~ O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RANGE_NUM: usize = 5;
    const RANGE: usize = Self::RANGE_NUM * 1000;
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let _len_ns: usize = nums.len();
        let mut ans: Vec<u8> = Vec::with_capacity(Self::RANGE);
        for num in nums {
            Self::get_digits(num, &mut ans);
        }
        return ans.into_iter().map(|num| num as i32).collect();
    }
    fn get_digits(num: i32, ans: &mut Vec<u8>) {
        let mut res: Vec<u8> = Vec::with_capacity(Self::RANGE_NUM);
        let mut num = num;
        while num > 0 {
            let digit = (num % 10) as u8;
            res.push(digit);
            num /= 10;
        }
        res.reverse();
        for n in res {
            ans.push(n);
        }
    }
}
