/// https://leetcode.com/problems/number-complement/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/number-complement/discuss/612591/Rust-simple-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let n: i32 = {
            let mut n: i32 = 1;
            while n < num{
                n = (n << 1) + 1;
            }
            n
        };
        n - num
    }
}