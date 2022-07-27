use std::cell::RefCell;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/design-a-food-rating-system/
/// Time Complexitiy:   N.A.
/// Space Complexity:   N.A.
/// Note - Lessons Learned:
/// https://stackoverflow.com/questions/58790368/updating-structs-inside-a-btreeset/58790477#58790477
/// https://stackoverflow.com/questions/53111721/can-i-modify-a-value-inside-a-binaryheap-that-isnt-the-top-value
#[allow(dead_code)]
struct FoodRatings {
    cuisine_to_foods: HashMap<String, BinaryHeap<Rc<RefCell<Food>>>>,
    name_to_food: HashMap<String, Rc<RefCell<Food>>>,
}

#[allow(dead_code)]
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let len_fs: usize = foods.len();
        let mut cuisine_to_foods: HashMap<String, BinaryHeap<Rc<RefCell<Food>>>> = HashMap::new();
        let mut name_to_food: HashMap<String, Rc<RefCell<Food>>> = HashMap::new();
        for idx in 0..len_fs {
            let name: String = foods[idx].to_owned();
            let cuisine: String = cuisines[idx].to_owned();
            let rating: i32 = ratings[idx];
            let food: Rc<RefCell<Food>> = Rc::new(RefCell::new(Food {
                name: name.to_owned(),
                rating,
            }));
            cuisine_to_foods
                .entry(cuisine)
                .or_default()
                .push(food.clone());
            name_to_food.insert(name.to_owned(), food.clone());
        }
        FoodRatings {
            cuisine_to_foods,
            name_to_food,
        }
    }

    fn change_rating(&mut self, name: String, new_rating: i32) {
        if let Some(food) = self.name_to_food.get(&name) {
            food.clone().borrow_mut().rating = new_rating;
        }
    }

    fn highest_rated(&self, cuisine: String) -> String {
        if let Some(heap) = self.cuisine_to_foods.get(&cuisine) {
            if let Some(top) = heap.peek() {
                return top.clone().borrow().name.to_owned();
            }
        }
        unreachable!();
    }
}

struct Food {
    name: String,
    rating: i32,
}

impl Ord for Food {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.rating, &self.name).cmp(&(other.rating, &other.name))
    }
}

impl PartialOrd for Food {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rating != other.rating {
            Some(self.rating.cmp(&other.rating))
        } else {
            Some(self.name.cmp(&other.name))
        }
    }
}

impl Eq for Food {}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.rating == other.rating
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[should_panic]
    fn test_with_sample_input_1_should_return_expected() {
        let foods: Vec<String> = vec![
            "kimchi".to_owned(),
            "miso".to_owned(),
            "sushi".to_owned(),
            "moussaka".to_owned(),
            "ramen".to_owned(),
            "bulgogi".to_owned(),
        ];
        let cuisines: Vec<String> = vec![
            "korean".to_owned(),
            "japanese".to_owned(),
            "japanese".to_owned(),
            "greek".to_owned(),
            "japanese".to_owned(),
            "korean".to_owned(),
        ];
        let ratings: Vec<i32> = vec![9, 12, 8, 15, 14, 7];
        let mut food_ratings: FoodRatings = FoodRatings::new(foods, cuisines, ratings);
        let expected1: String = "kimchi".to_owned();
        let actual1: String = food_ratings.highest_rated("korean".to_owned());
        assert_eq!(expected1, actual1);
        let expected2: String = "ramen".to_owned();
        let actual2: String = food_ratings.highest_rated("japanese".to_owned());
        assert_eq!(expected2, actual2);
        food_ratings.change_rating("sushi".to_owned(), 16);
        let expected3: String = "sushi".to_owned();
        let actual3: String = food_ratings.highest_rated("japanese".to_owned());
        assert_eq!(expected3, actual3);
        food_ratings.change_rating("ramen".to_owned(), 16);
        let expected4: String = "ramen".to_owned();
        let actual4: String = food_ratings.highest_rated("japanese".to_owned());
        assert_eq!(expected4, actual4);
    }
}
