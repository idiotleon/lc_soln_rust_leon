/// https://leetcode.com/problems/find-the-celebrity/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_celebrity(&self, n: i32) -> i32 {
        if n <= 1 {
            return -1;
        }
        let mut celebrity = 0;
        for i in 1..n {
            if self.knows(celebrity, i) {
                celebrity = i;
            }
        }
        for j in 0..n {
            if j == celebrity {
                continue;
            }
            if self.knows(celebrity, j) || !self.knows(j, celebrity) {
                return -1;
            }
        }
        celebrity
    }

    /// Fake API
    /// The knows API is defined for you.
    /// knows(a: i32, b: i32)->bool;
    /// to call it use self.knows(a,b)
    fn knows(&self, a: i32, b: i32) -> bool {
        a > b
    }
}
