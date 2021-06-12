/// https://leetcode.com/problems/minimum-number-of-refueling-stops/
/// 
/// Time Complexity:    O(`len_stns` ^ 2)
/// Space Complexity:   O(`len_stns`)
/// 
/// Reference:
/// https://leetcode.com/problems/minimum-number-of-refueling-stops/discuss/1266797/Rust%3A-4ms-Iterative-Memoization
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let len_stns = stations.len();
        
        let mut dp = vec![0; 1 + len_stns];
        dp[0] = start_fuel;
        
        for (idx, stn) in stations.iter().enumerate(){
            for t in (0..=idx).rev(){
                if dp[t] >= stn[0]{
                    dp[1 + t] = std::cmp::max(dp[1 + t], dp[t] + stn[1]);
                }
            }
        }
        
        dp.iter().position(|&stp| stp >= target).map_or(-1, |idx| idx as i32)
    }
}