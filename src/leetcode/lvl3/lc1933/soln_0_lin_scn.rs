/// https://leetcode.com/problems/check-if-string-is-decomposable-into-value-equal-substrings/
///
/// Time Complexity:    O(`len_p`)
/// Space Complexity:   O(`len_p`)
///
/// Reference:
/// https://leetcode.com/problems/check-if-string-is-decomposable-into-value-equal-substrings/discuss/1345047/C%2B%2B-solution-Note-the-rule-of-this-question-is-like-the-game-of-Mahjong.
/// https://leetcode.com/problems/check-if-string-is-decomposable-into-value-equal-substrings/discuss/1310512/Java-straightforward-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_decomposable(s: String) -> bool {
        let padded = format!("{}#", s);
        let len_p = padded.len();
        if len_p < 2 + 1 {
            return false;
        }

        let mut cnt = 1;
        // the flag whether value-equal substring of length two appeared
        let mut flag = false;

        let chs: Vec<char> = padded.chars().collect();

        for idx in 1..len_p {
            if chs[idx - 1] == chs[idx] {
                cnt += 1;
            } else {
                match cnt % 3 {
                    1 => return false,
                    2 => {
                        if flag {
                            return false;
                        }

                        flag = true;
                    }
                    _ => {}
                }
                cnt = 1;
            }
        }

        flag
    }
}
