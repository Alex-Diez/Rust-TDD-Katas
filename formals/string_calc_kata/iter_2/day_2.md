# String calculator formalization

## Evaluation function
1. Function `evaluate` should return either `Ok(f64)` or `ParseFloatError`
2. Function `evaluate` should accept add, sub, mul and div operations

## Input parameters
```
<math-expr> ::= <num>(<op><num>)*
<op> ::= {<add>|<sub>|<mul>|<div>}
<div> ::= ÷
<mul> ::= ×
<add> ::= +
<sub> ::= -
<num> ::= {<float-num>|<int-num>}
<float-num> ::= (0..9)*.(0..9)*
<int-num> ::= (0..9)*
```
