/// @author: Leon
/// https://leetcode.com/problems/the-latest-time-to-catch-a-bus/
/// Time Complexity:    O(`len_bs` + `len_ps`) ~ O(max(`len_bs`, `len_ps`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/the-latest-time-to-catch-a-bus/discuss/2259708/C%2B%2B-or-100-faster-or-Explanation-or-Beginner-Friendly-or-Easy-or-Unordered_set-or-O(n)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
        let len_bs: usize = buses.len();
        let len_ps: usize = passengers.len();
        let buses: Vec<i32> = {
            let mut buses = buses;
            buses.sort();
            buses
        };
        let passengers: Vec<i32> = {
            let mut ps = passengers;
            ps.sort();
            ps
        };
        let mut idx_b: usize = 0;
        let mut idx_p: usize = 0;
        let mut prev: i32 = -1;
        let mut ans: i32 = 0;
        while idx_b < len_bs {
            let mut cnt: i32 = 0;
            while cnt < capacity && idx_p < len_ps && passengers[idx_p] <= buses[idx_b] {
                if passengers[idx_p] - 1 != prev {
                    ans = passengers[idx_p] - 1;
                }
                prev = passengers[idx_p];
                cnt += 1;
                idx_p += 1;
            }
            if cnt < capacity && buses[idx_b] != prev {
                ans = buses[idx_b];
            }
            idx_b += 1;
        }
        ans
    }
}
