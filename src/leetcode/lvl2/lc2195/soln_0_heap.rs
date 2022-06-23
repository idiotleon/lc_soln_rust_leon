use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/append-k-integers-with-minimal-sum/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(`_len_n`)
/// !!!This is NOT a correct solution!!!
/// Reference:
/// https://leetcode.com/problems/append-k-integers-with-minimal-sum/discuss/1823630/n-*-(n-+-1)-2/1296158
/// https://leetcode.com/problems/append-k-integers-with-minimal-sum/discuss/1823630/n-*-(n-%2B-1)-2
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let _len_n: usize = nums.len();
        let mut heap: BinaryHeap<i32> = {
            let mut heap: BinaryHeap<i32> = BinaryHeap::new();
            for num in nums {
                heap.push(num);
            }
            heap
        };
        let mut ans: i64 = 0;
        let mut k: i64 = k as i64;
        let mut prev: i64 = 1;
        while !heap.is_empty() && k > 0 {
            let cur = heap.pop().unwrap() as i64;
            if prev < cur {
                if cur - prev <= k {
                    ans += ((cur - prev) * (prev + cur - 1)) / 2;
                    k -= cur - prev;
                } else {
                    break;
                }
            }
            prev = cur + 1;
        }
        ans += (k * (prev + prev + k - 1)) / 2;
        ans
    }
}
