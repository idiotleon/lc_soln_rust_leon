/// https://leetcode.com/problems/slowest-key/
///
/// Time Complexity:    O(len)
/// Space Complexity:   O(1)
///
/// Reference:
/// https://leetcode.com/problems/slowest-key/discuss/909535/Rust-0ms-100
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut prev: i32 = 0;
        let mut slowest: char = ' ';
        let mut longest: i32 = -1;

        for (idx, ch) in keys_pressed.chars().into_iter().enumerate() {
            let duration = release_times[idx] - prev;
            prev = release_times[idx];

            if duration > longest {
                longest = duration;
                slowest = ch;
            } else if duration == longest && ch > slowest {
                slowest = ch;
            }
        }
        slowest
    }
}
