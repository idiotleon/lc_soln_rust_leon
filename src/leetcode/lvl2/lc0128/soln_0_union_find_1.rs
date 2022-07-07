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
        let _len_ns: usize = nums.len();
        let mut roots: HashMap<i32, i32> = {
            let mut map: HashMap<i32, i32> = HashMap::new();
            for &num in &nums {
                map.insert(num, num);
            }
            map
        };
        let mut sizes: HashMap<i32, i32> = {
            let mut map: HashMap<i32, i32> = HashMap::new();
            for &num in &nums {
                map.insert(num, 1);
            }
            map
        };
        for &num in &nums {
            if roots.contains_key(&(num - 1)) {
                Self::union(num, num - 1, &mut roots, &mut sizes);
            }
            if roots.contains_key(&(num + 1)) {
                Self::union(num, num + 1, &mut roots, &mut sizes);
            }
        }
        sizes.into_values().max().unwrap_or(0)
    }
    fn union(x: i32, y: i32, roots: &mut HashMap<i32, i32>, sizes: &mut HashMap<i32, i32>) {
        let root_x: i32 = Self::find(x, roots);
        let root_y: i32 = Self::find(y, roots);
        if root_x == root_y {
            return;
        }
        let size_x: i32 = *sizes.get(&root_x).unwrap_or(&0);
        let size_y: i32 = *sizes.get(&root_y).unwrap_or(&0);
        if size_x > size_y {
            roots.insert(root_y, root_x);
            *sizes.entry(root_x).or_default() += size_y;
            *sizes.entry(y).or_default() = 0;
        } else {
            roots.insert(root_x, root_y);
            *sizes.entry(root_y).or_default() += size_x;
            *sizes.entry(x).or_default() = 0;
        }
    }
    fn find(mut x: i32, roots: &mut HashMap<i32, i32>) -> i32 {
        while x != *roots.get(&x).unwrap() {
            roots.insert(x, *roots.get(roots.get(&x).unwrap()).unwrap());
            x = *roots.get(&x).unwrap();
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
