# anita
Anita is a JIT compiler that allows runtime compilation of math expressions.

## Usage
Anita compiles a given expression and returns a structure with a function that follows a given signature. This can be achieved by using the `compile_expression!` macro.
```rust
let expression: String = "x+1".to_owned(); // This can be anything implementing AsRef<str>
let function: CompiledFunction<fn(f32) -> f32> = compile_expression!(expression, (x) -> f32);
assert_eq!(function(4_f32), 5_f32);
```

## Supported features
This is the current state of features in anita
### Types
So far anita only supports f32 floats.
Booleans are represented as floats where 0.0 => false and != 0.0 => true.
By default logical operators or functions return 1.0 if the result is true.

### Operators

Operator | Status | Description |
|----------|------------|-------------|
| ^   | supported | Exponentiation |
| *   | supported | Product |
| /   | supported | Division |
| %   | supported | Modulo |
| +   | supported | Sum |
| -   | supported | Difference |
| <   | supported | Lower than |
| \>  | supported | Greater than |
| <=  | supported | Lower than or equal |
| \>= | supported | Greater than or equal |
| ==  | supported | Equal |
| !=  | supported | Not equal |
| &&  | supported | Logical and |
| &#124;&#124; | supported | Logical or |
| =   | supported | Assignment |W
| ;   | supported | Expression Chaining |
| - (unary) | supported | Negation |
| !   | supported | Logical not |

### Functions
Anita includes the following Functions by default unless the `no-default-functions` feature is used.
| Identifier           | Status        | Argument Amount | Argument Types                | Description |
|----------------------|---------------|-----------------|-------------------------------|-------------|
| `min`                | supported     | 2               | Float                         | see [std::f32::min](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.min) |
| `max`                | supported     | 2               | Float                         | see [std::f32::max](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.max) |
| `floor`              | supported     | 1               | Float                         | see [std::f32::floor](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.floor) |
| `round`              | supported     | 1               | Float                         | see [std::f32::round](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.round) |
| `ceil`               | supported     | 1               | Float                         | see [std::f32::ceil](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.ceil) |
| `if`                 | supported     | 3               | Float, Float, Float           | If the first argument is normal and not equal to 0.0, returns the second argument, otherwise, returns the third  |
| `is_nan`             | supported     | 1               | Float                         | see [std::f32::is_nan](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.is_nan).<br> The return value is mapped to 1.0 if true and 0.0 if false |
| `is_finite`          | supported     | 1               | Float                         | see [std::f32::is_finite](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.is_finite).<br> The return value is mapped to 1.0 if true and 0.0 if false |
| `is_infinite`        | supported     | 1               | Float                         | see [std::f32::is_infinite](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.is_infinite).<br> The return value is mapped to 1.0 if true and 0.0 if false |
| `is_normal`          | supported     | 1               | Float                         | see [std::f32::is_normal](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.is_normal).<br> The return value is mapped to 1.0 if true and 0.0 if false |
| `ln`                 | supported     | 1               | Float                         | see [std::f32::ln](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.ln) |
| `log`                | supported     | 2               | Float, Float                  | see [std::f32::log](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.log) |
| `log2`               | supported     | 1               | Float                         | see [std::f32::log2](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.log2) |
| `log10`              | supported     | 1               | Float                         | see [std::f32::log10](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.log10) |
| `exp`                | supported     | 1               | Float                         | see [std::f32::exp](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.exp) |
| `exp2`               | supported     | 1               | Float                         | see [std::f32::exp2](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.exp2) |
| `pow`                | supported     | 2               | Float, Float                  | see [std::f32::pow](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.pow) |
| `cos`                | supported     | 1               | Float                         | see [std::f32::cos](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.cos) |
| `acos`               | supported     | 1               | Float                         | see [std::f32::acos](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.acos) |
| `cosh`               | supported     | 1               | Float                         | see [std::f32::cosh](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.cosh) |
| `acosh`              | supported     | 1               | Float                         | see [std::f32::acosh](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.acosh) |
| `sin`                | supported     | 1               | Float                         | see [std::f32::sin](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.sin) |
| `asin`               | supported     | 1               | Float                         | see [std::f32::asin](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.asin) |
| `sinh`               | supported     | 1               | Float                         | see [std::f32::sinh](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.sinh) |
| `asinh`              | supported     | 1               | Float                         | see [std::f32::asinh](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.asinh) |
| `tan`                | supported     | 1               | Float                         | see [std::f32::tan](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.tan) |
| `atan`               | supported     | 1               | Float                         | see [std::f32::atan](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.atan) |
| `atan2`              | supported     | 2               | Float, Float                  | see [std::f32::atan2](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.atan2) |
| `tanh`               | supported     | 1               | Float                         | see [std::f32::tanh](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.tanh) |
| `atanh`              | supported     | 1               | Float                         | Inverse hyperbolic tangent function. |
| `sqrt`               | supported     | 1               | Float                         | Returns the square root of a number. Returns NaN for a negative number |
| `cbrt`               | supported     | 1               | Float                         | Returns the cube root of a number |
| `hypot`              | supported     | 2               | Float                         | Calculates the length of the hypotenuse of a right-angle triangle given legs of length given by the two arguments |
| `abs`                | supported     | 1               | Float                         | Returns the absolute value of a float |

#### Custom Functions
If different functions are needed, they can be introduced using the `FunctionManager` trait.
To construct a function Manager it is advised to use the `#[function_manager]` macro attribute. 
By default using the custom functions will have the same name as they are declared with. This can be overwritten using the `#[name = "other_name"]` attribute.
```rust
struct CustomFunctions {}

#[function_manager]
impl CustomFunctions {
    fn custom(x: f32) -> f32 {
        x + 1.0
    }

    #[name = "not_zero"]
    fn custom_if_not_zero(x: f32) -> f32 {
        (x != 0.0) as u8 as f32
    }
}

#[cfg(tests)]
mod test {
    #[test]
    fn it_works() {
        let function: CompiledFunction<fn(f32) -> f32> = compile_expression!("not_zero(custom(x))", (x) -> f32);
        assert_eq(function(-1.0), 0.0);
        assert_eq(function(42.0), 1.0);
    }
}
```

## SIMD
TODO!

## Frontend
Anita uses a custom language frontend inspired by the [evalexpr](https://crates.io/crates/evalexpr) crate.

## Naming
The name anita is inspired by the first all-electronic desktop calculator [ANITA](<https://en.wikipedia.org/wiki/Sumlock_ANITA_calculator>)
