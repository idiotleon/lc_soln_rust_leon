use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/the-number-of-beautiful-subsets/
/// Time Complexity:    O(2 ^ `len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/the-number-of-beautiful-subsets/solutions/3363862/c-java-python-evolve-brute-force-to-dp-explained-7-approaches/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        const RANGE: usize = 1000;
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut seen: HashMap<i32, u16> = HashMap::with_capacity(len_ns);
        let mut visited: Vec<bool> = vec![false; len_ns];
        let mut path: Vec<i32> = Vec::with_capacity(len_ns);
        let mut paths: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(
            0,
            &sorted,
            k,
            &mut seen,
            &mut visited,
            &mut path,
            &mut paths,
        );
        return paths.len() as i32;
    }
    fn backtrack(
        idx_start: usize,
        nums: &Vec<i32>,
        k: i32,
        seen: &mut HashMap<i32, u16>,
        visited: &mut Vec<bool>,
        path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        let len_ns: usize = nums.len();
        if idx_start == len_ns {
            return;
        }
        if !path.is_empty() {
            paths.push(path.to_vec());
        }
        for idx in idx_start..len_ns {
            if *seen.entry(nums[idx] - k).or_default() > 0 || visited[idx] {
                continue;
            };
            *seen.entry(nums[idx]).or_default() += 1;
            visited[idx] = true;
            path.push(nums[idx]);
            Self::backtrack(idx, nums, k, seen, visited, path, paths);
            *seen.entry(nums[idx]).or_default() -= 1;
            visited[idx] = false;
            path.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![2, 4, 6];
        let k: i32 = 2;
        let expected: i32 = 4;
        let actual: i32 = Solution::beautiful_subsets(nums, k);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_1301() {
        let nums: Vec<i32> = vec![1, 1, 2, 3];
        let k: i32 = 1;
        let expected: i32 = 8;
        let actual: i32 = Solution::beautiful_subsets(nums, k);
        assert_eq!(expected, actual);
    }
}
