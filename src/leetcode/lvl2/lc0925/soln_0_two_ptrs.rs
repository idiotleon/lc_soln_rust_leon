/// @author: Leon
/// https://leetcode.com/problems/long-pressed-name/
/// Time Complexity:    O(`len_t`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let len_n: usize = name.len();
        let len_t: usize = typed.len();
        let chs_n: Vec<char> = name.chars().collect();
        let chs_t: Vec<char> = typed.chars().collect();
        let mut idx_n: usize = 0;
        for idx_t in 0..len_t {
            if idx_n < len_n && chs_n[idx_n] == chs_t[idx_t] {
                idx_n += 1;
            } else if idx_t == 0 || chs_t[idx_t] != chs_t[idx_t - 1] {
                return false;
            }
        }
        idx_n == len_n
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let name = "alex".to_owned();
        let typed = "aaleex".to_owned();
        let actual = Solution::is_long_pressed_name(name, typed);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
