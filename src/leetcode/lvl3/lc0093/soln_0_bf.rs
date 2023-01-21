/// @author: Leon
/// https://leetcode.com/problems/restore-ip-addresses/
/// Time Complexity:    O((4 ^ 4) * (~12) * `len_s`) ~ O(`len_s`)
/// Space Complexity:   O(4 ^ 4) ~ O(1)
/// Reference:
/// https://leetcode.com/problems/restore-ip-addresses/solutions/30972/who-can-beat-this-code/comments/29845
/// https://leetcode.com/problems/restore-ip-addresses/solutions/30972/who-can-beat-this-code/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let len_s: usize = s.len();
        const UPPER_BOUND: i32 = 256;
        const DOT: char = '.';
        let mut ans: Vec<String> = Vec::new();
        let mut ip: String = String::new();
        for a in 1..4 {
            for b in 1..4 {
                for c in 1..4 {
                    for d in 1..4 {
                        if a + b + c + d != len_s {
                            continue;
                        }
                        let n1: i32 = s[0..a].parse().unwrap_or(UPPER_BOUND);
                        let n2: i32 = s[a..a + b].parse().unwrap_or(UPPER_BOUND);
                        let n3: i32 = s[a + b..a + b + c].parse().unwrap_or(UPPER_BOUND);
                        let n4: i32 = s[a + b + c..].parse().unwrap_or(UPPER_BOUND);
                        if n1 < UPPER_BOUND
                            && n2 < UPPER_BOUND
                            && n3 < UPPER_BOUND
                            && n4 < UPPER_BOUND
                        {
                            ip.push_str(&n1.to_string());
                            ip.push(DOT);
                            ip.push_str(&n2.to_string());
                            ip.push(DOT);
                            ip.push_str(&n3.to_string());
                            ip.push(DOT);
                            ip.push_str(&n4.to_string());
                            if ip.len() == len_s + 3 {
                                ans.push(ip.to_owned());
                            }
                            ip.clear();
                        }
                    }
                }
            }
        }
        return ans;
    }
}
