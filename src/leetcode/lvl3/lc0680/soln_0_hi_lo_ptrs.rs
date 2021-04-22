/// https://leetcode.com/problems/valid-palindrome-ii/
///
/// Time Complexity:    O(`len`)
/// Space Complexity:   O(1)
///
/// Reference:
///     https://leetcode.com/problems/valid-palindrome-ii/discuss/107716/Java-O(n)-Time-O(1)-Space
impl Solution {
    pub fn valid_palindrome(str: String) -> bool {
        let len = str.len();
        let chs: Vec<char> = str.chars().collect();
        let mut lo: usize = 0;
        let mut hi: usize = len - 1;

        while (lo < hi) {
            if chs[lo] != chs[hi] {
                return Self::is_palindrome(lo + 1, hi, &chs)
                    || Self::is_palindrome(lo, hi - 1, &chs);
            }
            lo += 1;
            hi -= 1;
        }
        true
    }
    fn is_palindrome(low: usize, high: usize, chs: &Vec<char>) -> bool {
        let mut lo = low;
        let mut hi = high;

        while (lo < hi) {
            if chs[lo] != chs[hi] {
                return false;
            }
            lo += 1;
            hi -= 1;
        }
        true
    }
}
