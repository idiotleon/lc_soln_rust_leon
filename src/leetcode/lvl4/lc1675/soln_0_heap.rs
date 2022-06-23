use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/minimize-deviation-in-array/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(`_len_n`)
/// Reference:
/// https://leetcode.com/problems/minimize-deviation-in-array/discuss/952857/JavaC%2B%2BPython-Priority-Queue
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut min_odd: i32 = i32::MAX;
        let mut ans: i32 = i32::MAX;
        for num in nums {
            let odd = if num % 2 == 1 { num * 2 } else { num };
            heap.push(odd);
            min_odd = std::cmp::min(min_odd, odd);
        }
        loop {
            if let Some(top) = heap.pop() {
                ans = std::cmp::min(ans, top - min_odd);
                if top % 2 == 1 {
                    break;
                }
                min_odd = std::cmp::min(min_odd, top / 2);
                heap.push(top / 2);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        let actual = Solution::minimum_deviation(nums);
        let expected = 1;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let nums: Vec<i32> = vec![4, 1, 5, 20, 3];
        let actual = Solution::minimum_deviation(nums);
        let expected = 3;
        assert_eq!(expected, actual);
    }
}
