/// @author: Leon
/// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/
/// Time Complexity:    O(`n` * lg(`n`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/discuss/2612407/C%2B%2B-or-Diagram-or-Related-Problems
/// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/discuss/961350/C%2B%2B-O(N)-time-iterative
/// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/discuss/961446/Detailed-Thought-Process-with-Steps-Example-or-O(n)-Time-or-Java-8-1-Liner
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let n: i64 = n as i64;
        let mut ans: i64 = 0;
        let mut cnt: u32 = 0;
        for num in 1..=n {
            if num & (num - 1) == 0 {
                // to check if it is a power of 2
                cnt += 1;
            }
            ans = ((ans << cnt) % MOD + num) % MOD;
        }
        return ans as i32;
    }
}
