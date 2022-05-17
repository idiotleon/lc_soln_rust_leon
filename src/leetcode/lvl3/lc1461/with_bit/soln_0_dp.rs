/// @author: Leon
/// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(2 ^ `k`)
/// Reference:
/// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/discuss/1105774/Rust-0ms-one-line-64ms-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let _len_s: usize = s.len();
        let ku: usize = k as usize;
        let mut contained: Vec<bool> = vec![false; 1 << ku];
        let mut mask: usize = 0;
        let mut cnt_exp: u32 = 1 << k as u32;
        let mask_all_ones: usize = (1 << ku) - 1;
        for (idx, &b) in s.as_bytes().iter().enumerate() {
            mask = ((mask << 1) & mask_all_ones) + if b == b'1' { 1 } else { 0 };
            if idx + 1 < ku || contained[mask] {
                continue;
            }
            contained[mask] = true;
            cnt_exp -= 1;
            if cnt_exp == 0 {
                return true;
            }
        }
        false
    }
}
