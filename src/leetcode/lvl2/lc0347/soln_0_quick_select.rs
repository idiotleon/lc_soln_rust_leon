use rand::Rng;
use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/top-k-frequent-elements/description/
/// Time Complexity:    O(`len_ns`), O(`len_ns` ^ 2) in the worst case
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let k: usize = k as usize;
        let num_to_freq: HashMap<i32, u16> = {
            let mut map: HashMap<i32, u16> = HashMap::with_capacity(len_ns);
            for num in nums {
                *map.entry(num).or_default() += 1;
            }
            map
        };
        let mut distinct: Vec<i32> = {
            let mut res: Vec<i32> = Vec::with_capacity(num_to_freq.len());
            for (&num, _freq) in &num_to_freq {
                res.push(num);
            }
            res
        };
        let len_ds: usize = distinct.len();
        Self::quick_select(0, len_ds - 1, len_ds - k, &num_to_freq, &mut distinct);
        return distinct[len_ds - k..].to_vec();
    }
    fn quick_select(
        lo: usize,
        hi: usize,
        k: usize,
        num_to_freq: &HashMap<i32, u16>,
        distinct: &mut Vec<i32>,
    ) {
        if lo == hi {
            return;
        }
        let idx_pivot: usize = Self::partition(lo, hi, num_to_freq, distinct);
        if k == idx_pivot {
            return;
        } else if k < idx_pivot {
            Self::quick_select(lo, idx_pivot - 1, k, num_to_freq, distinct);
        } else {
            Self::quick_select(idx_pivot + 1, hi, k, num_to_freq, distinct);
        }
    }
    fn partition(
        lo: usize,
        hi: usize,
        num_to_freq: &HashMap<i32, u16>,
        distinct: &mut Vec<i32>,
    ) -> usize {
        let idx_pivot = lo + rand::thread_rng().gen_range(0..hi - lo + 1);
        let &freq_pivot = num_to_freq.get(&distinct[idx_pivot]).unwrap();
        distinct.swap(idx_pivot, hi);
        let mut idx_store = lo;
        for idx in lo..=hi {
            if let Some(&freq) = num_to_freq.get(&distinct[idx]) {
                if freq < freq_pivot {
                    distinct.swap(idx_store, idx);
                    idx_store += 1;
                }
            }
        }
        distinct.swap(idx_store, hi);
        return idx_store;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
        let k: i32 = 2;
        let expected: Vec<i32> = vec![1, 2];
        let actual: Vec<i32> = {
            let mut res = Solution::top_k_frequent(nums, k);
            res.sort();
            res
        };
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let nums: Vec<i32> = vec![1];
        let k: i32 = 1;
        let expected: Vec<i32> = vec![1];
        let actual: Vec<i32> = Solution::top_k_frequent(nums, k);
        assert_eq!(expected, actual);
    }
}
