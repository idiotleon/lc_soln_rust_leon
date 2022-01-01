/// https://leetcode.com/problems/burst-balloons/
/// Time Complexity:    O(`len_n` ^ 3)
/// Space Complexity:   O(`len_n` ^ 2)
/// Reference:
/// http://zxi.mytechroad.com/blog/dynamic-programming/leetcode-312-burst-balloons/
/// https://www.youtube.com/watch?v=z3hu2Be92UA
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let len_p: usize = len_n + 2;
        let padded: Vec<i32> = {
            let mut tmp: Vec<i32> = vec![0; len_p];
            tmp[0] = 1;
            tmp[len_n + 1] = 1;
            for (idx, num) in nums.into_iter().enumerate() {
                tmp[idx + 1] = num;
            }
            tmp
        };
        let mut memo = vec![vec![0; len_p]; len_p];
        Self::dfs(1, len_n, &padded, &mut memo)
    }
    fn dfs(lo: usize, hi: usize, padded: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if lo > hi {
            return 0;
        }
        if lo == hi {
            return padded[lo - 1] * padded[lo] * padded[lo + 1];
        }
        if memo[lo][hi] > 1 {
            return memo[lo][hi];
        }
        let mut max_coins = 0;
        for k in lo..=hi {
            max_coins = std::cmp::max(
                max_coins,
                Self::dfs(lo, k - 1, padded, memo)
                    + padded[lo - 1] * padded[k] * padded[hi + 1]
                    + Self::dfs(k + 1, hi, padded, memo),
            );
        }
        memo[lo][hi] = max_coins;
        max_coins
    }
}
