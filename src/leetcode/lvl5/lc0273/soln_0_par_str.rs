/// @author: Leon
/// https://leetcode.com/problems/integer-to-english-ans/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    const LESS_THAN_20: &'static [&'static str] = &[
        "",
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve",
        "Thirteen",
        "Fourteen",
        "Fifteen",
        "Sixteen",
        "Seventeen",
        "Eighteen",
        "Nineteen",
    ];
    const TENS: &'static [&'static str] = &[
        "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
    ];
    const THOUSANDS: &'static [&'static str] = &["", "Thousand", "Million", "Billion"];
    pub fn number_to_ans(num: i32) -> String {
        if num == 0 {
            return "Zero".to_owned();
        }
        let mut idx: usize = 0;
        let mut ans: String = String::new();
        let mut res = num;
        while res > 0 {
            if res % 1000 != 0 {
                ans = format!(
                    "{}{} {}",
                    Self::helper(res % 1000),
                    Self::THOUSANDS[idx],
                    ans
                );
            }
            res /= 1000;
            idx += 1;
        }
        return ans.trim().to_owned();
    }
    fn helper(num: i32) -> String {
        return if num == 0 {
            "".to_owned()
        } else if (1..20).contains(&num) {
            // there is a trailing space
            format!("{} ", Self::LESS_THAN_20[num as usize])
        } else if (20..100).contains(&num) {
            format!(
                "{} {}",
                Self::TENS[num as usize / 10],
                Self::helper(num % 10)
            )
        } else {
            format!(
                "{} Hundred {}",
                Self::LESS_THAN_20[num as usize / 100],
                Self::helper(num % 100)
            )
        };
    }
}
