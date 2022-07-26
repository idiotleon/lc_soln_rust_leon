use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

const MAX_LENGTH_FEED: usize = 10_usize;

/// @author: Leon
/// https://leetcode.com/problems/design-twitter/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/design-twitter/discuss/82935/Java-OOD-solution-with-detailed-explanation
/// https://leetcode.com/problems/design-twitter/discuss/82825/Java-OO-Design-with-most-efficient-function-getNewsFeed
/// Note:
/// this is not yet a correct solution.
#[derive(Default)]
struct Twitter {
    user_to_tweets: HashMap<i32, Vec<Tweet>>,
    follower_to_followees: HashMap<i32, HashSet<i32>>,
    timestamp: i32,
}

#[allow(dead_code)]
impl Twitter {
    fn new() -> Self {
        Default::default()
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.user_to_tweets
            .entry(user_id)
            .or_default()
            .push(Tweet::new(tweet_id, self.timestamp));
        Self::follow(self, user_id, user_id);
        self.timestamp += 1;
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        // a min heap
        let mut heap: BinaryHeap<Reverse<Tweet>> = BinaryHeap::new();
        if let Some(followees) = self.follower_to_followees.get(&user_id) {
            for followee_id in followees {
                if let Some(tweets) = self.user_to_tweets.get(&followee_id) {
                    for tweet in tweets {
                        if heap.len() < MAX_LENGTH_FEED {
                            heap.push(Reverse(tweet.clone()));
                        } else {
                            if let Some(Reverse(top)) = heap.peek() {
                                if tweet.timestamp <= top.timestamp {
                                    break;
                                } else {
                                    heap.push(Reverse(tweet.clone()));
                                    heap.pop();
                                }
                            }
                            // heap.push(Reverse(tweet.clone()));
                            // if heap.len() > MAX_LENGTH_FEED{
                            //     heap.pop();
                            // }
                        }
                    }
                }
            }
        }
        // heap.into_iter().map(|t| t.id).collect::<Vec<i32>>()
        let mut ans: Vec<i32> = Vec::with_capacity(heap.len());
        while let Some(Reverse(Tweet {
            id,
            timestamp: _timestamp,
        })) = heap.pop()
        {
            ans.push(id);
        }
        ans.reverse();
        ans
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follower_to_followees
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        // sanity check
        if follower_id == followee_id {
            return;
        }
        if let Some(followees) = self.follower_to_followees.get(&follower_id) {
            if followees.contains(&followee_id) {
                self.follower_to_followees
                    .entry(follower_id)
                    .or_default()
                    .remove(&follower_id);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Tweet {
    id: i32,
    timestamp: i32,
}

impl Tweet {
    pub fn new(id: i32, timestamp: i32) -> Self {
        Tweet { id, timestamp }
    }
}

impl Ord for Tweet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
    }
}

impl Eq for Tweet {}

impl PartialEq for Tweet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.timestamp == other.timestamp
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_test_case_10_should_return_expected() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        twitter.post_tweet(1, 3);
        twitter.post_tweet(1, 101);
        twitter.post_tweet(1, 13);
        twitter.post_tweet(1, 10);
        twitter.post_tweet(1, 2);
        twitter.post_tweet(1, 94);
        twitter.post_tweet(1, 505);
        twitter.post_tweet(1, 333);
        let expected: Vec<i32> = vec![333, 505, 94, 2, 10, 13, 101, 3, 5];
        let actual = twitter.get_news_feed(1);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_test_case_13_should_return_expected() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        twitter.post_tweet(1, 3);
        twitter.post_tweet(1, 101);
        twitter.post_tweet(1, 13);
        twitter.post_tweet(1, 10);
        twitter.post_tweet(1, 2);
        twitter.post_tweet(1, 94);
        twitter.post_tweet(1, 505);
        twitter.post_tweet(1, 333);
        twitter.post_tweet(1, 22);
        twitter.post_tweet(1, 11);
        let expected: Vec<i32> = vec![11, 22, 333, 505, 94, 2, 10, 13, 101, 3];
        let actual = twitter.get_news_feed(1);
        assert_eq!(expected, actual);
    }
}
