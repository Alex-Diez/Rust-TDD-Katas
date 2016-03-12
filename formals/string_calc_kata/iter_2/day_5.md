#String calculator formalization

## Evaluation function
1. `Calculator` should take `&str` and return `Ok(f64)` or `ParseFloatError`
2. fn `evaluate` should accept add, sub, mul and div operations
3. fn `evaluate` should accept number of operations
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
<num> ::= {<float>|<int>}
<float> ::= (0..9)*.(0..9)*
<int> ::= (0..9)*
```
