use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/longest-consecutive-sequence/description/
/// Time Complexity:    O(amortized(`_len_ns`))
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/longest-consecutive-sequence/discuss/166544/Union-Find-Thinking-Process
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let mut num_to_idx: HashMap<i32, usize> = HashMap::with_capacity(len_ns);
        let mut roots: Vec<usize> = {
            let mut roots: Vec<usize> = vec![0; len_ns];
            for idx in 0..len_ns {
                roots[idx] = idx;
            }
            roots
        };
        let mut sizes: Vec<i32> = vec![1; len_ns];
        for (idx, &num) in nums.iter().enumerate() {
            if num_to_idx.contains_key(&num) {
                continue;
            }
            if let Some(&idx_lo) = num_to_idx.get(&(num - 1)) {
                Self::union(idx, idx_lo, &mut roots, &mut sizes);
            }
            if let Some(&idx_hi) = num_to_idx.get(&(num + 1)) {
                Self::union(idx, idx_hi, &mut roots, &mut sizes);
            }
            num_to_idx.insert(num, idx);
        }
        sizes.into_iter().max().unwrap_or(0)
    }
    fn union(x: usize, y: usize, roots: &mut Vec<usize>, sizes: &mut Vec<i32>) {
        let root_x: usize = Self::find(x, roots);
        let root_y: usize = Self::find(y, roots);
        if root_x == root_y {
            return;
        }
        if sizes[root_x] > sizes[root_y] {
            roots[root_y] = root_x;
            sizes[root_x] += sizes[root_y];
            sizes[root_y] = 0;
        } else {
            roots[root_x] = root_y;
            sizes[root_y] += sizes[root_x];
            sizes[root_x] = 0;
        }
    }
    fn find(mut x: usize, roots: &mut Vec<usize>) -> usize {
        while x != roots[x] {
            roots[x] = roots[roots[x]];
            x = roots[x];
        }
        x
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![100, 4, 200, 1, 3, 2];
        let expected: i32 = 4;
        let actual: i32 = Solution::longest_consecutive(nums);
        assert_eq!(expected, actual)
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let nums: Vec<i32> = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let expected: i32 = 9;
        let actual: i32 = Solution::longest_consecutive(nums);
        assert_eq!(expected, actual)
    }
}
