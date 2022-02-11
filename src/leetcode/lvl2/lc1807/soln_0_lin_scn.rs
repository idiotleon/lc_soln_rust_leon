use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/evaluate-the-bracket-pairs-of-a-string/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_knwl` * avg_len_wd + `_len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let _len_s: usize = s.len();
        let _len_knwl: usize = knowledge.len();
        const OPEN_BRACKET: char = '(';
        const CLOSED_BRACKET: char = ')';
        const QUESTION_MARK: char = '?';
        let map: HashMap<String, String> = {
            let mut map: HashMap<String, String> = HashMap::new();
            for knwl in knowledge {
                map.insert(knwl[0].to_owned(), knwl[1].to_owned());
            }
            map
        };
        let mut ans: String = "".to_owned();
        let mut is_open: bool = false;
        let mut seg: String = "".to_owned();
        for ch in s.chars() {
            match ch {
                OPEN_BRACKET => {
                    is_open = true;
                }
                CLOSED_BRACKET => {
                    is_open = false;
                    if let Some(replace) = map.get(&seg) {
                        ans.push_str(replace);
                    } else {
                        ans.push(QUESTION_MARK);
                    }
                    seg.clear();
                }
                _ => {
                    if is_open {
                        seg.push(ch);
                    } else {
                        ans.push(ch);
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s: String = "(name)is(age)yearsold".to_owned();
        let knowledge: Vec<Vec<String>> = vec![
            vec!["name".to_owned(), "bob".to_owned()],
            vec!["age".to_owned(), "two".to_owned()],
        ];
        let actual: String = Solution::evaluate(s, knowledge);
        let expected: String = "bobistwoyearsold".to_owned();
        assert_eq!(expected, actual);
    }
}
