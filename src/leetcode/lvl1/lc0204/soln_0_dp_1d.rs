/// @author: Leon
/// https://leetcode.com/problems/count-primes/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
/// Reference:
/// https://leetcode.com/problems/count-primes/discuss/57588/My-simple-Java-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut not_prime = vec![false; n];
        let mut cnt = 0;

        for i in 2..n {
            if !not_prime[i] {
                cnt += 1;

                let mut j = 2;
                while i * j < n {
                    not_prime[i * j] = true;
                    j += 1;
                }
            }
        }

        return cnt;
    }
}
