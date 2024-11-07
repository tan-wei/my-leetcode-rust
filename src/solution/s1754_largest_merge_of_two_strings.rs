/**
 * [1754] Largest Merge Of Two Strings
 *
 * You are given two strings word1 and word2. You want to construct a string merge in the following way: while either word1 or word2 are non-empty, choose one of the following options:
 *
 * 	If word1 is non-empty, append the first character in word1 to merge and delete it from word1.
 *
 * 		For example, if word1 = "abc" and merge = "dv", then after choosing this operation, word1 = "bc" and merge = "dva".
 *
 *
 * 	If word2 is non-empty, append the first character in word2 to merge and delete it from word2.
 *
 * 		For example, if word2 = "abc" and merge = "", then after choosing this operation, word2 = "bc" and merge = "a".
 *
 *
 *
 * Return the lexicographically largest merge you can construct.
 * A string a is lexicographically larger than a string b (of the same length) if in the first position where a and b differ, a has a character strictly larger than the corresponding character in b. For example, "abcd" is lexicographically larger than "abcc" because the first position they differ is at the fourth character, and d is greater than c.
 *  
 * Example 1:
 *
 * Input: word1 = "cabaa", word2 = "bcaaa"
 * Output: "cbcabaaaaa"
 * Explanation: One way to get the lexicographically largest merge is:
 * - Take from word1: merge = "c", word1 = "abaa", word2 = "bcaaa"
 * - Take from word2: merge = "cb", word1 = "abaa", word2 = "caaa"
 * - Take from word2: merge = "cbc", word1 = "abaa", word2 = "aaa"
 * - Take from word1: merge = "cbca", word1 = "baa", word2 = "aaa"
 * - Take from word1: merge = "cbcab", word1 = "aa", word2 = "aaa"
 * - Append the remaining 5 a's from word1 and word2 at the end of merge.
 *
 * Example 2:
 *
 * Input: word1 = "abcabc", word2 = "abdcaba"
 * Output: "abdcabcabcaba"
 *
 *  
 * Constraints:
 *
 * 	1 <= word1.length, word2.length <= 3000
 * 	word1 and word2 consist only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-merge-of-two-strings/
// discuss: https://leetcode.com/problems/largest-merge-of-two-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_merge(word1: String, word2: String) -> String {
        let mut slice1 = word1.as_bytes();
        let mut slice2 = word2.as_bytes();
        let mut result = Vec::with_capacity(word1.len() + word2.len());

        while !slice1.is_empty() && !slice2.is_empty() {
            if slice1 > slice2 {
                result.push(slice1[0]);
                slice1 = &slice1[1..];
            } else {
                result.push(slice2[0]);
                slice2 = &slice2[1..];
            }
        }

        result.extend(slice1);
        result.extend(slice2);

        unsafe { String::from_utf8_unchecked(result) }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1754_example_1() {
        let word1 = "cabaa".to_string();
        let word2 = "bcaaa".to_string();

        let result = "cbcabaaaaa".to_string();

        assert_eq!(Solution::largest_merge(word1, word2), result);
    }

    #[test]
    fn test_1754_example_2() {
        let word1 = "abcabc".to_string();
        let word2 = "abdcaba".to_string();

        let result = "abdcabcabcaba".to_string();

        assert_eq!(Solution::largest_merge(word1, word2), result);
    }
}
