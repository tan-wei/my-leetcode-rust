/**
 * [0575] Distribute Candies
 *
 * Alice has n candies, where the i^th candy is of type candyType[i]. Alice noticed that she started to gain weight, so she visited a doctor.
 * The doctor advised Alice to only eat n / 2 of the candies she has (n is always even). Alice likes her candies very much, and she wants to eat the maximum number of different types of candies while still following the doctor's advice.
 * Given the integer array candyType of length n, return the maximum number of different types of candies she can eat if she only eats n / 2 of them.
 *  
 * Example 1:
 *
 * Input: candyType = [1,1,2,2,3,3]
 * Output: 3
 * Explanation: Alice can only eat 6 / 2 = 3 candies. Since there are only 3 types, she can eat one of each type.
 *
 * Example 2:
 *
 * Input: candyType = [1,1,2,3]
 * Output: 2
 * Explanation: Alice can only eat 4 / 2 = 2 candies. Whether she eats types [1,2], [1,3], or [2,3], she still can only eat 2 different types.
 *
 * Example 3:
 *
 * Input: candyType = [6,6,6,6]
 * Output: 1
 * Explanation: Alice can only eat 4 / 2 = 2 candies. Even though she can eat 2 candies, she only has 1 type.
 *
 *  
 * Constraints:
 *
 * 	n == candyType.length
 * 	2 <= n <= 10^4
 * 	n is even.
 * 	-10^5 <= candyType[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distribute-candies/
// discuss: https://leetcode.com/problems/distribute-candies/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        std::cmp::min(
            candy_type
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            candy_type.len() / 2,
        ) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0575_example_1() {
        let candy_type = vec![1, 1, 2, 2, 3, 3];
        let result = 3;

        assert_eq!(Solution::distribute_candies(candy_type), result);
    }

    #[test]
    fn test_0575_example_2() {
        let candy_type = vec![1, 1, 2, 3];
        let result = 2;

        assert_eq!(Solution::distribute_candies(candy_type), result);
    }

    #[test]
    fn test_0575_example_3() {
        let candy_type = vec![6, 6, 6, 6];
        let result = 1;

        assert_eq!(Solution::distribute_candies(candy_type), result);
    }
}
