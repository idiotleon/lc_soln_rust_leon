/// @author: Leon
/// https://leetcode.com/problems/count-integers-with-even-digit-sum/
/// Time Complexity:    O(`num` * lg(`num`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut cnt: i32 = 0;
        for n in 1..=num{
            if Self::get_sum(n) % 2 == 0{
                cnt += 1;
            }
        }
        cnt
    }
    fn get_sum(mut num: i32) -> i32{
        let mut sum: i32 = 0;
        while num > 0{
            sum += num % 10;
            num /= 10;
        }
        sum
    }
}