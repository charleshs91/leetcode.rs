/** 
 * Partition Equal Subset Sum
 * 
 * https://leetcode.com/problems/partition-equal-subset-sum/description/?envType=daily-question&envId=2025-04-07
 */
pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();
    println!("nums input={:?}, sum={:?}", nums, sum);

    // Two equal numbers can’t add up to an odd number.
    if sum % 2 != 0 {
        return false;
    }

    let target = sum as usize / 2;

    let mut dp = vec![false; target + 1];
    dp[0] = true;

    for &num in &nums {
        let num = num as usize;
        println!("outer loop; num={:?}", num);
        
        // If num >= target, this for-loop is ignored.
        for j in (num..=target).rev() {
            println!("  inner loop; num={:?}, j={:?}, j-num={:?}, dp[j]={:?}, dp[j-num]={:?}", num, j, j-num, dp[j], dp[j-num]);
            dp[j] = dp[j] | dp[j - num];
        }        
    }
    dp[target]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_partition_case1_true() {
        assert_eq!(can_partition(vec![1, 5, 11, 5]), true);
    }

    #[test]
    fn test_can_partition_case2_false() {
        assert_eq!(can_partition(vec![1, 2, 3, 5]), false);
    }

    #[test]
    fn test_can_partition_single_element_false() {
        assert_eq!(can_partition(vec![2]), false);
    }
}
