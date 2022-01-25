/// @author: Leon
/// https://leetcode.com/problems/sequential-digits/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for digit in 1..9 as i32 {
            let mut nxt = digit;
            let mut num = nxt;
            while num <= high && nxt < 10 {
                if num >= low {
                    ans.push(num);
                }
                nxt += 1;
                num = num * 10 + nxt;
            }
        }
        ans.sort();
        ans
    }
}
