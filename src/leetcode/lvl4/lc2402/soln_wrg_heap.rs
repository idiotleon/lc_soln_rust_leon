use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/meeting-rooms-iii/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let n: usize = n as usize;
        let mut freqs: Vec<i32> = vec![0; n];
        let mut heap: BinaryHeap<Node> = {
            let mut heap = BinaryHeap::with_capacity(n);
            for i in 0..n {
                heap.push(Node::new(0, i));
            }
            heap
        };
        for meeting in meetings {
            let start = meeting[0];
            let end = meeting[1];
            let duration = end - start;
            if let Some(Node { end: end_prev, idx }) = heap.pop() {
                heap.push(Node::new(std::cmp::max(end_prev, start) + duration, idx));
                freqs[idx] += 1;
            }
        }
        let mut most: i32 = 0;
        let mut ans: usize = 0;
        for (idx, freq) in freqs.into_iter().enumerate() {
            if freq > most {
                most = freq;
                ans = idx;
            }
        }
        return ans as i32;
    }
}

#[derive(Debug)]
struct Node {
    end: i32,
    // the index of the room
    idx: usize,
}

impl Node {
    pub fn new(end: i32, idx: usize) -> Self {
        Self { end, idx }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.end != other.end {
            return other.end.cmp(&self.end);
        }
        return other.idx.cmp(&self.idx);
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.end != other.end {
            return other.end.partial_cmp(&self.end);
        }
        return other.idx.partial_cmp(&self.idx);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.end == other.end && self.idx == other.idx;
    }
}

impl Eq for Node {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1_should_return_expected() {
        let n: i32 = 2;
        let meetings: Vec<Vec<i32>> = vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]];
        let expected: i32 = 0;
        let actual = Solution::most_booked(n, meetings);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_2_should_return_expected() {
        let n: i32 = 3;
        let meetings: Vec<Vec<i32>> =
            vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]];
        let expected: i32 = 1;
        let actual = Solution::most_booked(n, meetings);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_test_case_50_should_return_expected() {
        let n: i32 = 4;
        let meetings: Vec<Vec<i32>> = vec![
            vec![18, 19],
            vec![3, 12],
            vec![17, 19],
            vec![2, 13],
            vec![7, 19],
        ];
        let expected: i32 = 0;
        let actual = Solution::most_booked(n, meetings);
        assert_eq!(expected, actual);
    }
}
