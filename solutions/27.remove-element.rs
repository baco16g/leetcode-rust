/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut new_index = 0;
        for i in 0..nums.len() {
            if (nums[i] != val) {
                nums[new_index] = nums[i];
                new_index += 1;
            }
        }

        new_index as i32
    }
}
// @lc code=end
