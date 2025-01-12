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
This is the current status of evalexpr features in anita
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
| `min`                | supported     | 2               | Float                         | see [std::f32::max](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.max) |
| `max`                | supported     | 2               | Float                         | Returns the maximum of the arguments |
| `floor`              | supported     | 1               | Float                         | Returns the largest integer less than or equal to a number |
| `round`              | supported     | 1               | Float                         | Returns the nearest integer to a number. Rounds half-way cases away from 0.0 |
| `ceil`               | supported     | 1               | Float                         | Returns the smallest integer greater than or equal to a number |
| `if`                 | supported     | 3               | Float, Float, Float           | If the first argument is normal and not equal to 0.0, returns the second argument, otherwise, returns the third  |
| `is_nan`             | supported     | 1               | Float                         | Returns true if the argument is the floating-point value NaN, false if it is another floating-point value, and throws an error if it is not a number  |
| `is_finite`          | supported     | 1               | Float                         | Returns true if the argument is a finite floating-point number, false otherwise  |
| `is_infinite`        | supported     | 1               | Float                         | Returns true if the argument is an infinite floating-point number, false otherwise  |
| `is_normal`          | supported     | 1               | Float                         | Returns true if the argument is a floating-point number that is neither zero, infinite, [subnormal](https://en.wikipedia.org/wiki/Subnormal_number), or NaN, false otherwise  |
| `ln`                 | supported     | 1               | Float                         | Returns the natural logarithm of the number |
| `log`                | supported     | 2               | Float, Float                  | Returns the logarithm of the number with respect to an arbitrary base |
| `log2`               | supported     | 1               | Float                         | Returns the base 2 logarithm of the number |
| `log10`              | supported     | 1               | Float                         | Returns the base 10 logarithm of the number |
| `exp`                | supported     | 1               | Float                         | Returns `e^(number)`, (the exponential function) |
| `exp2`               | supported     | 1               | Float                         | Returns `2^(number)` |
| `pow`                | supported     | 2               | Float, Float                  | Raises a number to the power of the other number |
| `cos`                | supported     | 1               | Float                         | Computes the cosine of a number (in radians) |
| `acos`               | supported     | 1               | Float                         | Computes the arccosine of a number. The return value is in radians in the range [0, pi] or NaN if the number is outside the range [-1, 1] |
| `cosh`               | supported     | 1               | Float                         | Hyperbolic cosine function |
| `acosh`              | supported     | 1               | Float                         | Inverse hyperbolic cosine function |
| `sin`                | supported     | 1               | Float                         | Computes the sine of a number (in radians) |
| `asin`               | supported     | 1               | Float                         | Computes the arcsine of a number. The return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1] |
| `sinh`               | supported     | 1               | Float                         | Hyperbolic sine function |
| `asinh`              | supported     | 1               | Float                         | Inverse hyperbolic sine function |
| `tan`                | supported     | 1               | Float                         | Computes the tangent of a number (in radians) |
| `atan`               | supported     | 1               | Float                         | Computes the arctangent of a number. The return value is in radians in the range [-pi/2, pi/2] |
| `atan2`              | supported     | 2               | Float, Float                  | Computes the four quadrant arctangent in radians |
| `tanh`               | supported     | 1               | Float                         | Hyperbolic tangent function |
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
Anita uses a custom language frontend based on a reduced feature set of the [evalexpr](https://crates.io/crates/evalexpr) crate.

## Naming
The name anita is inspired by the first all-electronic desktop calculator [ANITA](<https://en.wikipedia.org/wiki/Sumlock_ANITA_calculator>)