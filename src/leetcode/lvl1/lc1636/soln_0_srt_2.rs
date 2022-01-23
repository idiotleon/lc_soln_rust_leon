use std::collections::{BinaryHeap, HashMap};
/// @author: Leon
/// https://leetcode.com/problems/sort-array-by-increasing-frequency/
/// Time Complexity:    O(`len_n` * lg(`l
/// en_n`))
/// Space Complexity:   O(1) / O(`len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        let num_to_freq: HashMap<i8, u8> = {
            let mut num_to_freq: HashMap<i8, u8> = HashMap::new();
            for &num in &nums {
                *num_to_freq.entry(num as i8).or_default() += 1;
            }
            num_to_freq
        };
        let mut heap: BinaryHeap<(i8, i8)> = BinaryHeap::new();
        for num in nums {
            heap.push((-(*num_to_freq.get(&(num as i8)).unwrap() as i8), num as i8));
        }
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = Vec::with_capacity(len_n);
            while let Some((_, num)) = heap.pop() {
                res.push(num as i32);
            }
            res
        };
        ans
    }
}
