use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/construct-target-array-with-multiple-sums/
/// Time Complexity:    O((`_len_ns`) * lg(`_len_ns`))
/// Space Complexity:   O(`_len_ns`)
/// Reference:
/// https://leetcode.com/problems/construct-target-array-with-multiple-sums/discuss/510256/JavaC++Python-Backtrack-OJ-is-wrong/499352
/// https://leetcode.com/problems/construct-target-array-with-multiple-sums/discuss/510256/JavaC%2B%2BPython-Backtrack-OJ-is-wrong
/// https://leetcode.com/problems/construct-target-array-with-multiple-sums/discuss/510256/JavaC%2B%2BPython-Backtrack-OJ-is-wrong
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let _len_ns: usize = target.len();
        let mut sum: i64 = 0;
        let mut heap: BinaryHeap<i64> = {
            let mut heap: BinaryHeap<i64> = BinaryHeap::new();
            for num in target {
                let num: i64 = num as i64;
                sum += num;
                heap.push(num);
            }
            heap
        };
        while let Some(top) = heap.pop() {
            sum -= top as i64;
            if top == 1 // all 1s
            // [1, idx] always is true
            || sum == 1
            {
                return true;
            }
            if top < sum || sum == 0 || top % sum == 0 {
                return false;
            }
            let top = top % sum;
            sum += top;
            heap.push(top);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let target = vec![9, 3, 5];
        let expected = true;
        let actual = Solution::is_possible(target);
        assert_eq!(expected, actual);
    }
}
