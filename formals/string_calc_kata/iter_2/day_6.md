# String calculator formalization

## Evaluation
1. fn `evaluate` should take `&str` and return `Ok(f64)` or `ParseFloatError`
2. fn `evaluate` should accept `add`, `sub`, `mul` and `div` operations
3. fn `evaluate` should accept number of different operations
4. fn `evaluate` should accept `(` and `)`

## Input data
```
<math-expr> ::= <term>({<add>|<sub>}<term>)*
<term> ::= <factor>({<mul>|<div>}<term>)*
<factor> ::= '('<math-expr>')'
<div> ::= '÷'
<mul> ::= '×'
<add> ::= '+'
<sub> ::= '-'
<num> ::= (0..9)*{.(0..9)*}
```
