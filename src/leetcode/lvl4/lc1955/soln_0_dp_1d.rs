/// https://leetcode.com/problems/count-number-of-special-subsequences/
/// 
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(3) ~ O(1)
/// 
/// Reference:
///  https://leetcode.com/problems/count-number-of-special-subsequences/discuss/1375485/JavaC%2B%2BPython-DP-Solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        // not used
        // let len_n = nums.len();

        const MOD: i32 = 1e9 as i32 + 7;
        
        let mut dp: Vec<i32> = vec![0; 3];
        
        for num in nums{
            let num = num as usize;
            dp[num] = (dp[num] + dp[num]) % MOD + if num > 0 { dp[num - 1] }else{ 1 };
            dp[num] %= MOD;
        }
        
        dp[2]
    }
}