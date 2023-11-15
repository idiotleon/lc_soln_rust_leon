/// @author: Leon
/// Time Complexity:    O(lg(`len_ns`))
/// Space Complexity:   O(1)
/// Reference:
/// https://github.com/phishman3579/java-algorithms-implementation/blob/master/src/com/jwetherell/algorithms/search/UpperBound.java
struct BinarySearchUpperBound;

#[allow(dead_code)]
impl BinarySearchUpperBound {
    fn upper_bound(nums: Vec<i32>, target: i32) -> isize {
        let len_ns: usize = nums.len();
        let mut lo: isize = 0;
        let mut hi: isize = len_ns as isize;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid as usize] <= target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return hi - 1;
    }
}
