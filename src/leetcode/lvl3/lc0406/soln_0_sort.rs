/// @author: Leon
/// https://leetcode.com/problems/queue-reconstruction-by-height/
/// Time Complexity:    O(`len_ps` ^ 2)
/// Space Complexity:   O(`len_ps`) / O(1)
/// Reference:
/// https://leetcode.com/problems/queue-reconstruction-by-height/discuss/89345/Easy-concept-with-PythonC%2B%2BJava-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_ps: usize = people.len();
        let sorted: Vec<Vec<i32>> = {
            let mut sorted = people;
            sorted.sort_by(|a, b| {
                if a[0] != b[0] {
                    b[0].partial_cmp(&a[0]).unwrap()
                } else {
                    a[1].partial_cmp(&b[1]).unwrap()
                }
            });
            sorted
        };
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(len_ps);
        for people in sorted {
            ans.insert(people[1] as usize, people);
        }
        ans
    }
}
