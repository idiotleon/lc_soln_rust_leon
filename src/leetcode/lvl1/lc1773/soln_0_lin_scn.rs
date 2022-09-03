/// @author: Leon
/// https://leetcode.com/problems/count-items-matching-a-rule/
/// Time Complexity:    O(`_len_is`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/count-items-matching-a-rule/discuss/1086218/C%2B%2B-2-liner-(count_if)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let _len_is: usize = items.len();
        const TYPE: &str = "type";
        const COLOR: &str = "color";
        const NAME: &str = "name";
        let idx: usize = match &rule_key as &str {
            TYPE => 0,
            COLOR => 1,
            NAME => 2,
            _ => unreachable!(),
        };
        return items
            .into_iter()
            .filter(|item| item[idx] == rule_value)
            .count() as i32;
    }
}
