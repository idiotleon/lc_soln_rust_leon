use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/maximum-performance-of-a-team/
/// Time Complexity:    O(`len_ts` * lg(`len_ts`))
/// Space Complexity:   O(`len_ts`)
/// Reference:
/// https://leetcode.com/problems/maximum-performance-of-a-team/discuss/539687/JavaC%2B%2BPython-Priority-Queue
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let len_ts: usize = n as usize;
        let k: usize = k as usize;
        let teams: Vec<(i32, i32)> = {
            let mut teams: Vec<(i32, i32)> = Vec::with_capacity(len_ts);
            for idx in 0..len_ts {
                teams.push((efficiency[idx], speed[idx]));
            }
            teams.sort_by_key(|&t| -t.0);
            teams
        };
        let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::with_capacity(len_ts);
        let mut max: i64 = 0;
        let mut sum: i64 = 0;
        for (ef, speed) in teams {
            heap.push(Reverse(speed as i64));
            sum += speed as i64;
            if heap.len() > k {
                if let Some(Reverse(top)) = heap.pop() {
                    sum -= top;
                }
            }
            max = std::cmp::max(max, sum * ef as i64);
        }
        return (max % MOD) as i32;
    }
}
