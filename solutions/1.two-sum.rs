/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(j) => return vec![*j, i as i32],
                None => {
                    map.insert(num, i as i32);
                }
            }
        }
        vec![]
    }
}
// @lc code=end
