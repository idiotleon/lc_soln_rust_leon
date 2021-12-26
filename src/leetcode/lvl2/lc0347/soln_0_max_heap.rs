use std::collections::{BinaryHeap, HashMap};

/// https://leetcode.com/problems/top-k-frequent-elements/
/// Time Complexity:    O()
/// Space Complexity:   O()
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let num_to_freq: HashMap<i32, u16> = {
            let mut tmp: HashMap<i32, u16> = HashMap::new();
            for &num in &nums {
                *tmp.entry(num).or_insert(0) += 1;
            }
            tmp
        };
        let mut heap: BinaryHeap<(i16, i32)> = BinaryHeap::new();
        for (num, freq) in num_to_freq.into_iter() {
            heap.push((-(freq as i16), num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        let ans: Vec<i32> = {
            let mut idx: usize = 0;
            let mut tmp: Vec<i32> = vec![0; k as usize];
            while let Some((_, num)) = heap.pop() {
                tmp[idx] = num;
                idx += 1;
            }
            tmp.reverse();
            tmp
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
        let actual = Solution::top_k_frequent(nums, k);
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
