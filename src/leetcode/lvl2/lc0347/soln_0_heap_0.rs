use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// @author: Leon
/// https://leetcode.com/problems/top-k-frequent-elements/
/// Time Complexity:    O(`len_ns` * lg(`k`))
/// Space Complexity:   O(`k`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let k: usize = k as usize;
        let num_to_freq: HashMap<i32, u32> = {
            let mut map: HashMap<i32, u32> = HashMap::with_capacity(len_ns);
            for num in nums {
                *map.entry(num).or_default() += 1;
            }
            map
        };
        let mut heap: BinaryHeap<Reverse<(u32, i32)>> = BinaryHeap::with_capacity(len_ns);
        for (num, freq) in num_to_freq.into_iter() {
            heap.push(Reverse((freq, num)));
            if heap.len() > k {
                heap.pop();
            }
        }
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = Vec::with_capacity(k);
            while let Some(Reverse((_freq, num))) = heap.pop() {
                res.push(num);
            }
            res
        };
        return ans;
    }
}
