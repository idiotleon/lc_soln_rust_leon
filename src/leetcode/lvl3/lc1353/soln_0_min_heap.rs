use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/
/// Time Complexity:    O(`RANGE` + `len_ets` * lg(`len_ets`))
/// Space Complexity:   O(`len_ets`)
/// Reference:
/// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/discuss/510263/JavaC++Python-Priority-Queue/451931
/// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/discuss/510263/JavaC%2B%2BPython-Priority-Queue
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        const RANGE: i32 = 1e5 as i32 + 7;
        let len_ets: usize = events.len();
        let sorted = {
            let mut sorted = events;
            sorted.sort_by_key(|e| e[0]);
            sorted
        };
        let mut ans: i32 = 0;
        let mut idx: usize = 0;
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for d in 1..RANGE {
            while idx < len_ets && sorted[idx][0] == d {
                heap.push(Reverse(sorted[idx][1]));
                idx += 1;
            }
            while let Some(&Reverse(top)) = heap.peek() {
                if top < d {
                    heap.pop();
                } else {
                    break;
                }
            }
            if !heap.is_empty() {
                ans += 1;
                heap.pop();
            }
        }
        ans
    }
}
