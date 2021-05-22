/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low_index = 0;
        let mut high_index = nums.len() - 1;
        let mut mid_index: usize;
        let mut mid: i32;
        let mut last_index = 0;

        while low_index <= high_index {
            mid_index = (low_index + high_index) / 2;
            mid = nums[mid_index];
            if mid < target {
                low_index = (mid_index + 1) as usize;
                last_index = low_index;
            } else if mid_index == 0 {
                return 0i32;
            } else if mid > target {
                high_index = (mid_index - 1) as usize;
                last_index = low_index;
            } else {
                return mid_index as i32;
            }
        }

        last_index as i32
    }
}
// @lc code=end
