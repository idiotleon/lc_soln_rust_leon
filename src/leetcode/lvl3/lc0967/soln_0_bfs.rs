/// @author: Leon
/// https://leetcode.com/problems/numbers-with-same-consecutive-differences/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1) / O(`n`)
/// Reference:
/// https://leetcode.com/problems/numbers-with-same-consecutive-differences/discuss/211183/JavaC%2B%2BPython-Iterative-BFS-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut cur: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for _ in 2..=n {
            let mut nxt: Vec<i32> = Vec::with_capacity(k as usize);
            for num in cur {
                let d = num % 10;
                if d + k < 10 {
                    nxt.push(num * 10 + d + k);
                }
                if k > 0 && d - k >= 0 {
                    nxt.push(num * 10 + d - k);
                }
            }
            cur = nxt;
        }
        return cur;
    }
}
