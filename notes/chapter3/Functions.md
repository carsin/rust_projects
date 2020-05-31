# 3.2 Functions

Rust's conventional naming style is *snake case*: `fn example_function()`

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

In function signatures, you *must* declare the type of each parameter.

Function bodies contain statements and expressions

*   *Statements*: perform some action and do not return a value. 

*   *Expressions*: evaluate to a resulting value.

## Functions with Return Values

Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (`->`).

```rust
fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // No semicolon because it’s an expression whose value we want to return.
}
```

You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly.

