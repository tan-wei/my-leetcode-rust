/**
 * [1376] Time Needed to Inform All Employees
 *
 * A company has n employees with a unique ID for each employee from 0 to n - 1. The head of the company is the one with headID.
 * Each employee has one direct manager given in the manager array where manager[i] is the direct manager of the i-th employee, manager[headID] = -1. Also, it is guaranteed that the subordination relationships have a tree structure.
 * The head of the company wants to inform all the company employees of an urgent piece of news. He will inform his direct subordinates, and they will inform their subordinates, and so on until all employees know about the urgent news.
 * The i-th employee needs informTime[i] minutes to inform all of his direct subordinates (i.e., After informTime[i] minutes, all his direct subordinates can start spreading the news).
 * Return the number of minutes needed to inform all the employees about the urgent news.
 *  
 * Example 1:
 *
 * Input: n = 1, headID = 0, manager = [-1], informTime = [0]
 * Output: 0
 * Explanation: The head of the company is the only employee in the company.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/27/graph.png" style="width: 404px; height: 174px;" />
 * Input: n = 6, headID = 2, manager = [2,2,-1,2,2,2], informTime = [0,0,1,0,0,0]
 * Output: 1
 * Explanation: The head of the company with id = 2 is the direct manager of all the employees in the company and needs 1 minute to inform them all.
 * The tree structure of the employees in the company is shown.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	0 <= headID < n
 * 	manager.length == n
 * 	0 <= manager[i] < n
 * 	manager[headID] == -1
 * 	informTime.length == n
 * 	0 <= informTime[i] <= 1000
 * 	informTime[i] == 0 if employee i has no subordinates.
 * 	It is guaranteed that all the employees can be informed.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/time-needed-to-inform-all-employees/
// discuss: https://leetcode.com/problems/time-needed-to-inform-all-employees/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for (i, &m) in manager.iter().enumerate() {
            if m != -1 {
                graph[m as usize].push(i);
            }
        }
        let mut result = 0;
        let mut stack = vec![(head_id as usize, 0)];
        while let Some((node, time)) = stack.pop() {
            result = result.max(time);
            for &child in &graph[node] {
                stack.push((child, time + inform_time[node]));
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1376_example_1() {
        let n = 1;
        let head_id = 0;
        let manager = vec![-1];
        let inform_time = vec![0];

        let result = 0;

        assert_eq!(
            Solution::num_of_minutes(n, head_id, manager, inform_time),
            result
        );
    }

    #[test]
    fn test_1376_example_2() {
        let n = 6;
        let head_id = 2;
        let manager = vec![2, 2, -1, 2, 2, 2];
        let inform_time = vec![0, 0, 1, 0, 0, 0];

        let result = 1;

        assert_eq!(
            Solution::num_of_minutes(n, head_id, manager, inform_time),
            result
        );
    }
}
