/// @author: Leon
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
/// Reference:
/// https://github.com/phishman3579/java-algorithms-implementation/blob/master/src/com/jwetherell/algorithms/search/LowerBound.java
struct BinarySearchLowerBound;

#[allow(dead_code)]
impl BinarySearchLowerBound {
    fn lower_bound(nums: Vec<i32>, target: i32) -> isize {
        let len_ns: usize = nums.len();
        let mut lo: isize = 0;
        let mut hi: isize = len_ns as isize;
        while lo < hi {
            let mid: isize = lo + (hi - lo) / 2;
            if nums[mid as usize] >= target {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        return lo;
    }
}
