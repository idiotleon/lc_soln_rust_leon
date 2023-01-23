use super::fake_array_reader::FakeArrayReader;

/// @author: Leon
/// https://leetcode.com/problems/find-the-index-of-the-large-integer/
/// Time Complexity:    O(lg(`len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_index(reader: &FakeArrayReader) -> i32 {
        let len_ns: i32 = reader.length();
        let mut len: i32 = len_ns;
        let mut lo: i32 = 0;
        while len > 1 {
            len /= 2;
            let res = reader.compare_sub(lo, lo + len - 1, lo + len, lo + len + len - 1);
            if res == 0 {
                return lo + len + len;
            } else if res < 0 {
                lo += len;
            }
        }
        return lo;
    }
}
