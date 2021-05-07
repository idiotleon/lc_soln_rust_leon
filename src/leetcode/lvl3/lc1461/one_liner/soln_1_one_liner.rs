/// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/
/// 
/// Time Complexity:    O()
/// Space Complexity:   O()
/// 
/// Reference:
/// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/discuss/1105774/Rust-0ms-one-line-64ms-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {

    pub fn has_all_codes(s: String, k: i32) -> bool {
        s.as_bytes()
            .windows(k as usize)
            .collect::<std::collections::HashSet<_>>()
            .len()
            == 1 << k
    }
}
