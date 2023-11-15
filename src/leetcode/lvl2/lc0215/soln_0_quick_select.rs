/// @author: Leon
/// https://leetcode.com/problems/kth-largest-element-in-an-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let len_ns = nums.len();
        let k: usize = k as usize;
        let k_largest = len_ns - k;
        let mut lo: usize = 0;
        let mut hi: usize = len_ns - 1;
        while lo <= hi {
            let pivot = Self::partition(lo, hi, &mut nums);
            if pivot < k_largest {
                lo = pivot + 1;
            } else if pivot > k_largest {
                hi = pivot - 1;
            } else {
                return nums[pivot];
            }
        }
        return -1;
    }
    fn partition(low: usize, high: usize, nums: &mut Vec<i32>) -> usize {
        if low == high {
            return low;
        }
        let mut lo: usize = low;
        let mut hi: usize = high + 1;
        let pivot = nums[low];
        loop {
            lo += 1;
            while nums[lo] < pivot {
                if lo == high {
                    break;
                }
                lo += 1;
            }
            hi -= 1;
            while pivot < nums[hi] {
                if hi == low {
                    break;
                }
                hi -= 1;
            }
            if lo >= hi {
                break;
            }
            nums.swap(lo, hi);
        }
        nums.swap(low, hi);
        return hi;
    }
}
