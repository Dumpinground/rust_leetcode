/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 * [242] 有效的字母异位词
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        for us in s.as_bytes() {
            let count = map.entry(us).or_insert(0);
            *count += 1;
        }
        for ut in t.as_bytes() {
            let count = map.entry(ut).or_insert(0);
            *count -= 1;
        }
        for u in map {
            if u.1 != 0_i16 {
                return false;
            }
        }

        true
    }
}
// @lc code=end
fn main() {
    Solution::is_anagram("ssnd".to_owned(), "tough".to_owned());
    let st = vec![
        ("end", "dne"),
        ("send", "dent"),
        ("right", "a"),
        ("anagram", "nagaram"),
        ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbba")
    ];
    for i in st {
        println!(
            "{} and {} are {} anagram",
            i.0,
            i.1,
            if Solution::is_anagram(i.0.to_string(), i.1.to_string()) {
                ""
            } else {
                "not"
            }
        );
    }
}
