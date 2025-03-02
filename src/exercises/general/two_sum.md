# Two Sum

## Problem Description

Write a function that takes an array of numbers (integers for the tests) and a target number. It should find two different items in the array that, when added together, give the target value. The indices of these items should then be returned in a tuple like so: `(index1, index2)`.

## Constraints

- The input will always be valid (numbers will be an array of length 2 or greater, and all of the items will be numbers)
- The target will always be the sum of two different items from the array
- For some tests, there may be multiple valid solutions; any valid solution will be accepted

## Examples

```rust
two_sum(&[1, 2, 3], 4) // should return (0, 2) or (2, 0)
two_sum(&[1234, 5678, 9012], 14690) // should return (1, 2) or (2, 1)
two_sum(&[2, 2, 3], 4) // should return (0, 1) or (1, 0)
```

## Function Signature

```rust
fn two_sum(numbers: &[i32], target: i32) -> (usize, usize)
```

## Implementation Notes

A common approach is to use a hash map to store the complement of each number and its index. This allows for an O(n) time complexity solution:

1. Iterate through the array
2. For each number, calculate its complement (target - number)
3. Check if the complement exists in the hash map
4. If it does, return the index of the complement and the current index
5. If not, add the current number and its index to the hash map

## Edge Cases

- If no solution is found, the function should panic with an appropriate message

## Based On

This problem is based on: <http://oj.leetcode.com/problems/two-sum/>
