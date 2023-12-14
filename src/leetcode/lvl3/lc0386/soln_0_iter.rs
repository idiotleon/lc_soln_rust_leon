/// @author: Leon
/// https://leetcode.com/problems/lexicographical-numbers/
/// Time Complexity:    O(`n`)
/// Space Complexiyt:   O(1)
/// Reference:
/// https://leetcode.com/problems/lexicographical-numbers/solutions/86237/ac-200ms-c-solution-beats-98/
/// https://leetcode.com/problems/lexicographical-numbers/solutions/86242/java-o-n-time-o-1-space-iterative-solution-130ms/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity(n as usize);
        let mut cur: i32 = 1;
        for _ in 0..n {
            ans.push(cur);
            if cur * 10 <= n {
                cur *= 10;
            } else {
                if cur >= n {
                    cur /= 10;
                }
                cur += 1;
                while cur % 10 == 0 {
                    cur /= 10;
                }
            }
        }
        return ans;
    }
}
