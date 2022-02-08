/// @author: Leon
/// https://leetcode.com/problems/add-digits/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        Self::dfs(num)
    }
    fn dfs(num: i32) -> i32 {
        if num < 10 {
            return num;
        }
        let mut num = num;
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        Self::dfs(sum)
    }
}
