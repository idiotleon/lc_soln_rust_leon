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
        let k = k as usize;
        let mut contained = vec![false; 1 << k];
        let (mut decimal, mut exp_cnt) = (0, 1 << k);
        let mask = (1 << k) - 1;

        for (i, &b) in s.as_bytes().iter().enumerate() {
            decimal = ((decimal << 1) & mask) + if b == b'1' { 1 } else { 0 };
            if i + 1 < k || contained[decimal] {
                continue;
            }
            contained[decimal] = true;
            exp_cnt -= 1;
            if exp_cnt == 0 {
                return true;
            }
        }
        false
    }
}
