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
Anita can support any type implementing the `AnitaType` trait but only can support one type at a time which is speccified by the return type in the `compile_expression!` macro.

### Operators

  Operator | Description |
|----------|-------------|
| ^   | Exponentiation |
| *   | Product |
| /   | Division |
| %   | Modulo |
| +   | Sum |
| -   | Difference |
| <   | Lower than |
| \>  | Greater than |
| <=  | Lower than or equal |
| \>= | Greater than or equal |
| ==  | Equal |
| !=  | Not equal |
| &&  | Logical and |
| &#124;&#124; | Logical or |
| =   | Assignment |
| ;   | Expression Chaining |
| - (unary) | supported | Negation |
| !   | Logical not |

### Functions
Anita ships with a set of default functions for the f32 type. If these are not used the `no-default-functions` feature can be enabled to reduce compiler overhead.
| Identifier           | Argument Amount | Description |
|----------------------|-----------------|-------------|
| `min`                | 2               | see [core::f32::min](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.min) |
| `max`                | 2               | see [core::f32::max](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.max) |
| `floor`              | 1               | see [std::f32::floor](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.floor) |
| `round`              | 1               | see [std::f32::round](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.round) |
| `ceil`               | 1               | see [std::f32::ceil](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.ceil) |
| `if`                 | 3               | If the first argument is normal and not equal to 0.0, returns the second argument, otherwise, returns the third  |
| `is_nan`             | 1               | see [core::f32::is_nan](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.is_nan).<br> The return value is mapped to 1.0 if true and 0.0 if false |
| `is_finite`          | 1               | see [core::f32::is_finite](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.is_finite).<br> The return value is mapped to 1.0 if true and 0.0 if false |
| `is_infinite`        | 1               | see [core::f32::is_infinite](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.is_infinite).<br> The return value is mapped to 1.0 if true and 0.0 if false |
| `is_normal`          | 1               | see [core::f32::is_normal](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.is_normal).<br> The return value is mapped to 1.0 if true and 0.0 if false |
| `mod`                | 2               | see [core::f32::rem](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.rem) |
| `ln`                 | 1               | see [std::f32::ln](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.ln) |
| `log`                | 2               | see [std::f32::log](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.log) |
| `log2`               | 1               | see [std::f32::log2](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.log2) |
| `log10`              | 1               | see [std::f32::log10](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.log10) |
| `exp`                | 1               | see [std::f32::exp](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.exp) |
| `exp2`               | 1               | see [std::f32::exp2](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.exp2) |
| `pow`                | 2               | see [std::f32::pow](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.pow) |
| `cos`                | 1               | see [std::f32::cos](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.cos) |
| `acos`               | 1               | see [std::f32::acos](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.acos) |
| `cosh`               | 1               | see [std::f32::cosh](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.cosh) |
| `acosh`              | 1               | see [std::f32::acosh](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.acosh) |
| `sin`                | 1               | see [std::f32::sin](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.sin) |
| `asin`               | 1               | see [std::f32::asin](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.asin) |
| `sinh`               | 1               | see [std::f32::sinh](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.sinh) |
| `asinh`              | 1               | see [std::f32::asinh](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.asinh) |
| `tan`                | 1               | see [std::f32::tan](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.tan) |
| `atan`               | 1               | see [std::f32::atan](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.atan) |
| `atan2`              | 2               | see [std::f32::atan2](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.atan2) |
| `tanh`               | 1               | see [std::f32::tanh](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.tanh) |
| `atanh`              | 1               | see [std::f32::atanh](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.atanh) |
| `sqrt`               | 1               | see [std::f32::sqrt](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.sqrt) |
| `cbrt`               | 1               | see [std::f32::cbrt](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.cbrt) |
| `hypot`              | 2               | see [std::f32::hypot](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.hypot) |
| `abs`                | 1               | see [core::f32::abs](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.abs) |

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
        (x != 0) as u8 as f32
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