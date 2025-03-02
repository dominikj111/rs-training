# Are Two Arrays Same

Given two arrays `a` and `b`, write a function `comp(a, b)` that checks whether the two arrays have the "same" elements, with the same multiplicities (the multiplicity of a member is the number of times it appears). "Same" means, here, that the elements in `b` are the elements in `a` squared, regardless of the order.

## Examples

### Valid arrays

```rust
a = [121, 144, 19, 161, 19, 144, 19, 11]
b = [121, 14641, 20736, 361, 25921, 361, 20736, 361]
```

`comp(a, b)` returns `true` because in `b`:

- 121 is the square of 11
- 14641 is the square of 121
- 20736 is the square of 144
- 361 is the square of 19
- 25921 is the square of 161
- and so on.

It gets obvious if we write b's elements in terms of squares:

```rust
a = [121, 144, 19, 161, 19, 144, 19, 11]
b = [11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19]
```

### Invalid arrays

If, for example, we change the first number to something else, `comp` is not returning true anymore:

```rust
a = [121, 144, 19, 161, 19, 144, 19, 11]
b = [132, 14641, 20736, 361, 25921, 361, 20736, 361]
```

`comp(a,b)` returns `false` because in `b`, 132 is not the square of any number of `a`.

```rust
a = [121, 144, 19, 161, 19, 144, 19, 11]
b = [121, 14641, 20736, 36100, 25921, 361, 20736, 361]
```

`comp(a,b)` returns `false` because in `b`, 36100 is not the square of any number of `a`.

## Remarks

- `a` or `b` might be `[]` or `{}`.
- `a` or `b` might be `nil` or `null` or `None` or nothing.
- If `a` or `b` are `nil` (or `null` or `None`, depending on the language), the problem doesn't make sense so return `false`.

## Function Signature

```rust
fn comp(a: Vec<i64>, b: Vec<i64>) -> bool
```

## Implementation Notes

A clever solution approach:

```rust
// Not my, but clever solution.
let mut a1 = a.iter().map(|&x| x * x).collect::<Vec<_>>();
let mut a2 = b;
a1.sort();
a2.sort();
a1 == a2
```
