use std::collections::BinaryHeap;
/// @author: Leon
/// https://leetcode.com/problems/find-k-closest-elements/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(`k`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_closest_elements(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let _len_n: usize = nums.len();
        // max heap
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        for num in nums {
            heap.push(((num - x).abs(), num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = Vec::new();
            while let Some((_diff, num)) = heap.pop() {
                res.push(num);
            }
            res.sort();
            res
        };
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let k: i32 = 4;
        let x: i32 = 3;
        let actual = Solution::find_closest_elements(nums, k, x);
        let expected = vec![1, 2, 3, 4];
        assert_eq!(expected, actual);
    }
}
