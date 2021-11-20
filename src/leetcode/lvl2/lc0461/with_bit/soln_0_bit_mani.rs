/// https://leetcode.com/problems/hamming-distance/
/// Time Complexity:    O(bit_one(`x` ^ `y`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/hamming-distance/discuss/94698/Java-1-Line-Solution-%3AD
/// https://tech.liuchao.me/2016/11/count-bits-of-integer/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        Self::get_hamming_weight(x ^ y)
    }
    fn get_hamming_weight(mut num: i32) -> i32 {
        let cnt = {
            let mut cnt = 0;
            while num != 0 {
                cnt += num & 1;
                num >>= 1;
            }
            cnt
        };
        cnt
    }
}
