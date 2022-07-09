use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/evaluate-division/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let len_eqs: usize = equations.len();
        let mut roots: HashMap<&str, &str> = HashMap::new();
        let mut dist: HashMap<&str, f64> = HashMap::new();
        for (idx, query) in queries.iter().enumerate() {
            let numerator = &query[0];
            let divisor = &query[1];
            Self::union(numerator, divisor, &mut roots, &mut dist, values[idx]);
        }
        let mut ans: Vec<f64> = vec![0.0; len_eqs];
        for (idx, eq) in equations.iter().enumerate() {
            let numerator = &eq[0].as_str();
            let divisor = &eq[1].as_str();
            if !roots.contains_key(numerator)
                || !roots.contains_key(divisor)
                || !Self::find(numerator, &mut roots, &mut dist)
                    .ne(Self::find(divisor, &mut roots, &mut dist))
            {
                ans[idx] = -1.0;
            } else {
                ans[idx] = dist.get(numerator).unwrap() / dist.get(divisor).unwrap();
            }
        }
        ans
    }
    fn union<'a>(
        numerator: &'a str,
        divisor: &'a str,
        roots: &'a mut HashMap<&'a str, &'a str>,
        dist: &'a mut HashMap<&'a str, f64>,
        value: f64,
    ) {
        *roots.entry(numerator).or_insert(numerator);
        *dist.entry(numerator).or_insert(1.0);
        *roots.entry(divisor).or_insert(divisor);
        *dist.entry(divisor).or_insert(1.0);
        let root_n: &str = Self::find(numerator, &mut roots, &mut dist);
        let root_d: &str = Self::find(divisor, &mut roots, &mut dist);
        roots.insert(root_n, root_d);
        dist.insert(
            root_n,
            value * dist.get(root_d).unwrap() / dist.get(root_n).unwrap(),
        );
    }
    fn find<'a>(
        x: &'a str,
        roots: &'a mut HashMap<&'a str, &'a str>,
        dist: &'a mut HashMap<&'a str, f64>,
    ) -> &'a str {
        if let Some(&rt) = roots.get(x) {
            if rt == x {
                return x;
            }
            let rt_grand = Self::find(rt, roots, dist);
            roots.insert(rt, &rt_grand);
            dist.insert(rt, dist.get(rt).unwrap() * dist.get(rt_grand).unwrap());
            return rt_grand;
        } else {
            roots.insert(x, x);
            dist.insert(x, 1.0);
            return x;
        }
    }
}
