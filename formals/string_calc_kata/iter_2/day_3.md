# String calculator formalization

## Evaluation function

1. Function `evaluate` should return Ok(f64) or ParseFloatError
2. Function `evaluate` should accept add, sub, mul and div operation

## Input value
```
<math-expr> ::= <num>(<oper><num>)*
<oper> :: {<add>|<sub>|<mul>|<div>}
<add> ::= +
<sub> ::= -
<mul> ::= ×
<div> ::= ÷
<num> ::= {<int-num>|<float-num>}
<int-num> ::= (0..9)*
<float-num> ::= (0..9)*.(0..9)
```
