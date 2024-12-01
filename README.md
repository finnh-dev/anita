# anita
Anita is a JIT compiler that allows runtime compilation of expressions in the format of [evalexpr](https://github.com/ISibboI/evalexpr).

## Usage
Anita compiles a given expression and returns a structure with a function that follows a given signature. This can be achieved by using the `compile_expression!` macro.
```rust
let function = compile_expression!("x+1", (x) -> f32);
assert_eq!(function.execute(4_f32), 5_f32);
```

## Supported features
This is the current status of evalexpr features in anita
### Types
So far anita only supports floats and uses implicit casts for function calls.
Type | Status |
|----------|------------|
| f32 | supported |
| f64 | planned |
| bool | planned |
| i32 | unsupported |
| i64 | unsupported |
| String | unsupported |
| Tupel | unsupported |

### Operators
Operator | Status | Description |
|----------|------------|-------------|
| ^ | work in progress | Exponentiation |
| * | supported | Product |
| / | supported | Division |
| % | planned | Modulo |
| + | supported | Sum |
| - | supported | Difference |
| < | planned | Lower than |
| \> | planned | Greater than |
| <= | planned | Lower than or equal |
| \>= | planned | Greater than or equal |
| == | planned | Equal |
| != | planned | Not equal |
| && | planned | Logical and |
| &#124;&#124; | planned | Logical or |
| = | supported | Assignment |
| += | planned | Sum-Assignment or String-Concatenation-Assignment |
| -= | planned | Difference-Assignment |
| *= | planned | Product-Assignment |
| /= | planned | Division-Assignment |
| %= | planned | Modulo-Assignment |
| ^= | planned | Exponentiation-Assignment |
| &&= | planned | Logical-And-Assignment |
| &#124;&#124;= | planned | Logical-Or-Assignment |
| , | unsupported | Aggregation |
| ; | planned | Expression Chaining |
| - (unary) | supported | Negation |
| ! | unsupported | Logical not |

### Other features
Evalexpr allows the declaration of new variables. This is supported using anita but all used variables have to either be mentioned in the function signature or used with the assignment operator before being read to avoid the use of uninitialized variables.

## SIMD
TODO!

## Safety

## Naming
The name anita is inspired by the first all-electronic desktop calculator [Anita](<https://en.wikipedia.org/wiki/Sumlock_ANITA_calculator>)