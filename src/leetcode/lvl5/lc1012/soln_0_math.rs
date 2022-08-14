/// @author: Leon
/// https://leetcode.com/problems/numbers-with-repeated-digits/
/// Time Complexity:    O(lg(`n`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/numbers-with-repeated-digits/discuss/256725/JavaPython-Count-the-Number-Without-Repeated-Digit/251730
/// https://leetcode.com/problems/numbers-with-repeated-digits/discuss/256725/JavaPython-Count-the-Number-Without-Repeated-Digit
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let digits: Vec<i32> = {
            let mut nums: Vec<i32> = Vec::new();
            let mut num = n + 1;
            while num > 0 {
                nums.push(num % 10);
                num /= 10;
            }
            nums.reverse();
            nums
        };
        let len_ds: usize = digits.len();
        let mut cnt_dist: i32 = 0;
        for idx in 0..len_ds - 1 {
            cnt_dist += 9 * Self::permute(9, idx);
        }
        let mut is_occupied: Vec<bool> = vec![false; 10];
        for (idx, digit) in digits.into_iter().enumerate() {
            for d in if idx == 0 { 1 } else { 0 }..digit {
                if is_occupied[d as usize] {
                    continue;
                } else {
                    cnt_dist += Self::permute(10 - (idx as i32 + 1), len_ds - idx - 1);
                }
            }
            if is_occupied[digit as usize] {
                break;
            }
            is_occupied[digit as usize] = true;
        }
        return n - cnt_dist;
    }
    fn permute(mut n: i32, c: usize) -> i32 {
        let mut ans = 1;
        for _ in 0..c {
            ans *= n;
            n -= 1;
        }
        return ans;
    }
}
