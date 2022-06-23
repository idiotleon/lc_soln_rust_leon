/// @author: Leon
/// https://leetcode.com/problems/minimum-number-of-refueling-stops/
/// Time Complexity:    O(`len_stns` * lg(`len_stns`))
/// Space Complexity:   O(`len_stns`)
/// Reference:
/// https://leetcode.com/problems/minimum-number-of-refueling-stops/discuss/1266797/Rust%3A-4ms-Iterative-Memoization
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let len_stns = stations.len();
        let mut heap = BinaryHeap::<i32>::new();
        let mut i = 0;
        let mut fewest = 0;
        let mut start_fuel = start_fuel;
        while start_fuel < target {
            while i < len_stns && stations[i][0] <= start_fuel {
                heap.push(stations[i][1]);
                i += 1;
            }
            if heap.is_empty() {
                return -1;
            }
            start_fuel += heap.pop().unwrap();
            fewest += 1;
        }
        fewest
    }
}
