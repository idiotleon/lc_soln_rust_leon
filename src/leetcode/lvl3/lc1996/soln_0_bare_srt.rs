/// @author: Leon
/// https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/
/// Time Complexity:    O(`_len_ps` * lg(`_len_ps`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/discuss/1445186/EASY-C%2B%2B-solution-with-great-explanation-and-comments-(nlogn)-sorting
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let _len_ps: usize = properties.len();
        let properties: Vec<Vec<i32>> = {
            let mut nums = properties;
            nums.sort_by(|a, b| {
                if a[0] == b[0] {
                    b[1].partial_cmp(&a[1]).unwrap()
                } else {
                    a[0].partial_cmp(&b[0]).unwrap()
                }
            });
            nums
        };
        let mut mtn: i32 = 1;
        let mut cnt: i32 = 0;
        for property in properties.into_iter().rev() {
            let defense = property[1];
            if defense < mtn {
                cnt += 1;
            }
            mtn = std::cmp::max(mtn, defense);
        }
        return cnt;
    }
}
