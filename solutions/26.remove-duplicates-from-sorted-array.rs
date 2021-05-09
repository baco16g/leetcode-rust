/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut new_index = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[new_index] {
                new_index += 1;
                nums[new_index] = nums[i];
            }
        }

        (new_index + 1) as i32
    }
}
// @lc code=end
