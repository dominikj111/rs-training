# Break Camel Case

## Problem Description

Complete the function that breaks up camel casing, using a space between words.

## Examples

```rust
"camelCasing"  =>  "camel Casing"
"identifier"   =>  "identifier"
""             =>  ""
```

## Function Signature

```rust
fn break_camelcase(s: &str) -> String
```

## Implementation Notes

A possible approach:

1. Iterate through each character in the string
2. If the character is uppercase, insert a space before it
3. Add the character to the result string
4. Return the final string

## Test Cases

- `break_camelcase("camelCasing")` should return `"camel Casing"`
- `break_camelcase("identifier")` should return `"identifier"`
- `break_camelcase("")` should return `""`
- `break_camelcase("camelCasingTest")` should return `"camel Casing Test"`
