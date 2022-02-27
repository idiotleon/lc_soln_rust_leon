/// @author: Leon
/// https://leetcode.com/problems/minimum-time-to-complete-trips/
/// Time Complexity:    O(lg(`RANGE`) * `_len_ts`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_time(times: Vec<i32>, total_trips: i32) -> i64 {
        let _len_ts: usize = times.len();
        const RANGE: i64 = 1e14 as i64 + 7;
        let total_trips: i64 = total_trips as i64;
        let mut lo: i64 = 1;
        let mut hi: i64 = RANGE;
        while lo < hi{
            let mid = lo + (hi - lo) / 2;
            if Self::get_trips(mid, &times) >= total_trips{
                hi = mid;
            }else{
                lo = mid + 1;
            }
        }
        lo as i64
    }
    fn get_trips(guess: i64, times: &Vec<i32>) -> i64{
        let mut trips: i64 = 0;
        for &time in times{
            trips += guess / time as i64;
        }
        trips
    }
}