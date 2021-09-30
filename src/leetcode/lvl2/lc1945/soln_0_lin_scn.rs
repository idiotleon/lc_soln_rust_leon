/// https://leetcode.com/problems/sum-of-digits-of-string-after-convert/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/problems/sum-of-digits-of-string-after-convert/discuss/1360766/Handle-k..z-first/1023425
/// https://leetcode.com/problems/sum-of-digits-of-string-after-convert/discuss/1360766/Handle-k..z-first
/// https://leetcode.com/problems/sum-of-digits-of-string-after-convert/discuss/1360727/C%2B%2B-Simulation
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let init: i32 = {
            let mut sum = 0;
            for ch in s.chars() {
                let num = ch as u16 - 'a' as u16 + 1;
                sum += num % 10 + num / 10;
            }
            sum as i32
        };

        let mut k = k - 1;
        let mut sum = init;
        while k > 0 && sum > 9 {
            let mut tmp = 0;
            while sum > 0 {
                tmp += sum % 10;
                sum /= 10;
            }
            sum = tmp;
            k -= 1;
        }

        sum
    }
}
