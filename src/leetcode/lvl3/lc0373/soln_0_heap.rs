use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/
/// Time Complexity:    O(`k` * lg(`k`))
/// Space Complexity:   O(`k`)
/// Reference:
/// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/solutions/84551/simple-java-o-klogk-solution-with-explanation/comments/150089
/// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/solutions/84551/simple-java-o-klogk-solution-with-explanation/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let len1: usize = nums1.len();
        let len2: usize = nums2.len();
        // usize overflow
        // let mut ans: Vec<Vec<i32>> = Vec::with_capacity(len1 * len2);
        let mut ans: Vec<Vec<i32>> = Vec::new();
        if nums1.is_empty() || nums2.is_empty() || k == 0 {
            return ans;
        }
        let k: usize = k as usize;
        // (sum, num1, num2, idx2)
        let mut heap: BinaryHeap<Reverse<(i32, i32, i32, usize)>> = {
            // usize overflow
            // let mut heap: BinaryHeap<Reverse<(i32, i32, i32, usize)>> = BinaryHeap::with_capacity(len1 * len2);
            let mut heap: BinaryHeap<Reverse<(i32, i32, i32, usize)>> = BinaryHeap::new();
            for idx in 0..std::cmp::min(len1, k) {
                let sum = nums1[idx] + nums2[0];
                heap.push(Reverse((sum, nums1[idx], nums2[0], 0)));
            }
            heap
        };
        let mut cnt: usize = 0;
        while let Some(Reverse((_sum, num1, num2, idx2))) = heap.pop() {
            ans.push(vec![num1, num2]);
            if idx2 < len2 - 1 {
                let sum = num1 + nums2[idx2 + 1];
                heap.push(Reverse((sum, num1, nums2[idx2 + 1], idx2 + 1)));
            }
            cnt += 1;
            if cnt >= k {
                break;
            }
        }
        return ans;
    }
}
