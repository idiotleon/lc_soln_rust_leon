use std::collections::{HashMap, HashSet, VecDeque};

const MAX_LENGTH_TWEET: usize = 10;

/// @author: Leon
/// https://leetcode.com/problems/design-twitter/
/// Time Complexities:
///     `new()`:            O(1)
///     `post_tweet()`:     O(1)
///     `get_news_feed`:    O(O(len_twts))
///     `follow`:           O(1)
///     `unfollow`:         O(1)
/// Space Complexity:       O(O(len_twts) + O(len_flws))
/// Reference:
/// https://leetcode.com/problems/design-twitter/discuss/82837/Java-Solutions-with-Two-Maps-and-PriorityQueue/87118
/// https://leetcode.com/problems/design-twitter/discuss/82837/Java-Solutions-with-Two-Maps-and-PriorityQueue
#[derive(Debug, Default)]
struct Twitter {
    tweets: VecDeque<Tweet>,
    follower_to_followees: HashMap<i32, HashSet<i32>>,
}

#[allow(dead_code)]
impl Twitter {
    fn new() -> Self {
        Default::default()
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push_front(Tweet { user_id, tweet_id })
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let followees = self.follower_to_followees.entry(user_id).or_default();
        let mut ans: Vec<i32> = Vec::with_capacity(MAX_LENGTH_TWEET);
        for tweet in self.tweets.iter() {
            if ans.len() == MAX_LENGTH_TWEET {
                break;
            }
            let user_id_tweet = tweet.user_id;
            if user_id_tweet == user_id || followees.contains(&user_id_tweet) {
                ans.push(tweet.tweet_id);
            }
        }
        ans
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follower_to_followees
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follower_to_followees
            .entry(follower_id)
            .or_default()
            .remove(&followee_id);
    }
}

#[derive(Debug, Default)]
struct Tweet {
    user_id: i32,
    tweet_id: i32,
}
