/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().map_or(0, |v| v.len() as i32)
    }
}
// @lc code=end
