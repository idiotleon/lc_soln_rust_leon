use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/constrained-subsequence-sum/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/constrained-subsequence-sum/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::with_capacity(len_ns);
        heap.push((nums[0], 0));
        let mut ans: i32 = nums[0];
        for i in 1..len_ns {
            while let Some(&top) = heap.peek() {
                if i as i32 - top.1 > k {
                    heap.pop();
                } else {
                    break;
                }
            }
            let cur = std::cmp::max(0, heap.peek().unwrap().0) + nums[i];
            ans = std::cmp::max(ans, cur);
            heap.push((cur, i as i32));
        }
        return ans;
    }
}
