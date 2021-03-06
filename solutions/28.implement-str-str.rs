/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
 */

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let h_len = haystack.len();
        let n_len = needle.len();
        if (needle.is_empty()) {
            return 0;
        }
        if h_len < n_len {
            return -1;
        }

        for i in 0..(h_len - n_len + 1) {
            if haystack[i..(i + n_len)] == needle {
                return i as i32;
            }
        }

        -1
    }
}
// @lc code=end
