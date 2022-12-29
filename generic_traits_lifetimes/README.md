# Generic types, traits and lifetimes

See: https://doc.rust-lang.org/book/ch10-00-generics.html

## Performance of Code Using Generics

You might be wondering whether there is a runtime cost when using generic type parameters. The good news is that using generic types won't make your program run any slower than it would with concrete types.

Rust accomplishes this by performing **`monomorphization`** of the code using generics at compile time. **`Monomorphization`** is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

Let’s look at how this works by using the standard library’s generic `Option<T>` enum:
```rust
let integer = Some(5);
let float = Some(5.0);
```
When Rust compiles this code, it performs **`monomorphization`**. During that process, the compiler reads the values that have been used in `Option<T>` instances and identifies two kinds of `Option<T>`: one is `i32` and the other is `f64`. As such, it expands the generic definition of `Option<T>` into two definitions specialized to `i32` and `f64`, thereby replacing the generic definition with the specific ones.

The monomorphized version of the code looks similar to the following (the compiler uses different names than what we’re using here for illustration):

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
The generic `Option<T>` is replaced with the specific definitions created by the compiler. Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of **`monomorphization`** makes Rust’s generics extremely efficient at runtime.

## Blanket implementations

We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called `blanket implementations` and are extensively used in the Rust standard library.

For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait. The `impl` block in the standard library looks similar to this code:

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

