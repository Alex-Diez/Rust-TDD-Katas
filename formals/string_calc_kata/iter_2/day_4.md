# String calculator formalization

## Evaluation function
1. Function `evaluate` should take `&str` as parameter and return `Ok(f64)` or `ParseFloatError`
2. Function `evaluate` should accept add, sub and mul operation

## Input value
```
<math-expr> ::= <num>(<op><num>)*
<op> ::= {<add>|<sub>|<mul>|<div>}
<div> ::= ÷
<mul> ::= ×
<add> ::= +
<sub> ::= -
<num> ::= {<int-num>|<float-num>}
<int-num> ::= (0..9)*
<float-num> ::= (0..9)*.(0..9)*
```
