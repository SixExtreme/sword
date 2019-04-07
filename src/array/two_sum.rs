use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hmap: HashMap<i32, i32> = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            if !hmap.contains_key(x) {
                hmap.insert(target - x, i as i32);
            } else {
                return vec![*hmap.get(x).unwrap(), i as i32];
            }
        }
        return vec![-1, -1];
    }
}

#[test]
fn test_solution() {
    let (nums, target) = (vec![2, 7, 11, 15], 9);
    let ans = Solution::two_sum(nums, target);
    assert_eq!(ans, vec![0, 1]);
}