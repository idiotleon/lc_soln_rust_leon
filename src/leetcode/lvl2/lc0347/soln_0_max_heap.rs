use std::collections::{BinaryHeap, HashMap};

/// https://leetcode.com/problems/top-k-frequent-elements/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(`_len_n`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let num_to_freq: HashMap<i32, u16> = {
            let mut num_to_freq: HashMap<i32, u16> = HashMap::new();
            for num in nums {
                *num_to_freq.entry(num).or_default() += 1;
            }
            num_to_freq
        };
        let mut heap: BinaryHeap<(i16, i32)> = BinaryHeap::new();
        for (num, freq) in num_to_freq.into_iter() {
            heap.push((-(freq as i16), num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        let ans: Vec<i32> = {
            let mut ans: Vec<i32> = Vec::new();
            while let Some((_freq, num)) = heap.pop() {
                ans.push(num);
            }
            ans
        };
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let actual = {
            let mut sorted: Vec<i32> = Solution::top_k_frequent(nums, k);
            sorted.sort();
            sorted
        };
        let expected = vec![1, 2];
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_sample_input_2() {
        let nums = vec![1];
        let k = 1;
        let actual = Solution::top_k_frequent(nums, k);
        let expected = vec![1];
        assert_eq!(actual, expected);
    }
}
