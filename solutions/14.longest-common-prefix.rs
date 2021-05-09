/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let first = match strs.first() {
            Some(v) => v.to_string(),
            None => return String::new(),
        };
        strs.iter().skip(0).fold(first, |acc, str| {
            acc.chars()
                .zip(str.chars())
                .take_while(|(a, b)| a == b)
                .map(|(x, _)| x)
                .collect::<String>()
        })
    }
}
// @lc code=end
