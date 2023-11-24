/// @author: Leon
/// https://leetcode.com/problems/fizz-buzz/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let n: usize = n as usize;
        let mut ans: Vec<String> = Vec::with_capacity(n);
        for num in 1..=n {
            let res = if num % 3 == 0 && num % 5 == 0 {
                "FizzBuzz".to_owned()
            } else if num % 3 == 0 {
                "Fizz".to_owned()
            } else if num % 5 == 0 {
                "Buzz".to_owned()
            } else {
                num.to_string()
            };
            ans.push(res);
        }
        return ans;
    }
}
