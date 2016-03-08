# String calculator formalization

## Evaluation function

1. For each `&str` function `evaluate` should return `Ok(f32)` or `Err(ParseFloatError)`
2. `evaluate` should work with addition and subtraction in any order and any numbers

## Input values
```
math-expr ::= <term>|<term>({<add>|<sub>}<term>)
term ::= (0...9)*.(0...9)
add ::= +
sub ::= -
```
