# Computing

## Incremental computing

- [Salsa](https://github.com/salsa-rs/salsa) - A generic framework for on-demand, incrementalized computation. Inspired by adapton, glimmer, and rustc's query system.
  - **Inputs**: the base inputs to your system. You can change these whenever you like.
  - **Functions**: pure functions (no side effects) that transform your inputs into other values. The results of queries are memoized to
  avoid recomputing them a lot. When you make changes to the inputs, we'll figure out (fairly intelligently) when we can re-use these
  memoized values and when we have to recompute them.
- [Adapton in Rust](https://github.com/Adapton/adapton.rust) - A general-purpose Incremental Computation (IC) library for Rust.
  - Adapton offers programming language abstractions for incremental computation.
- [Anchors](https://github.com/lord/anchors) - self adjusting computations in rust.
  - Hybrid graph allows both [Adapton](https://github.com/Adapton/adapton.rust)-style and [Incremental](https://github.com/janestreet/incremental)-style push updates. For more information on the internals, you can view the [accompanying blog post](https://lord.io/blog/2020/spreadsheets/).
  - Cloning values in the graph is almost always optional. `map` and `then` closures receive immutable references, and return owned values. Alternatively, a `refmap` closure receives an immutable reference, and returns an immutable reference.
  - Still a work in progress, but should be functional (lol) and half-decently fast. Still, expect for there to be major API changes over the next several years.



## Math Eval

### [meval](https://github.com/rekka/meval-rs)

This Rust crate provides a simple math expression parsing and evaluation. Its main goal is to be convenient to use, while allowing for some flexibility. Currently works only with f64 types. A typical use case is the configuration of numerical computations in Rust, think initial data and boundary conditions, via config files or command line arguments.

```rust
fn main() {
    let r = meval::eval_str("1 + 2").unwrap();

    println!("1 + 2 = {}", r);
}
```

### [evalexpr](https://github.com/ISibboI/evalexpr)

Evalexpr is an expression evaluator and tiny scripting language in Rust. It has a small and easy to use interface and can be easily integrated into any application. It is very lightweight and comes with no further dependencies. Evalexpr is available on crates.io, and its API Documentation is available on docs.rs.

```rust
use evalexpr::*;

assert_eq!(eval("1 + 2 + 3"), Ok(Value::from(6)));
// `eval` returns a variant of the `Value` enum,
// while `eval_[type]` returns the respective type directly.
// Both can be used interchangeably.
assert_eq!(eval_int("1 + 2 + 3"), Ok(6));
assert_eq!(eval("1 - 2 * 3"), Ok(Value::from(-5)));
assert_eq!(eval("1.0 + 2 * 3"), Ok(Value::from(7.0)));
assert_eq!(eval("true && 4 > 2"), Ok(Value::from(true)));
```

### library: [fasteval](https://github.com/likebike/fasteval)

```rust
fn main() -> Result<(), fasteval::Error> {
    // This example doesn't use any variables, so just use an EmptyNamespace:
    let mut ns = fasteval::EmptyNamespace;

    let val = fasteval::ez_eval(
        "1+2*3/4^5%6 + log(100K) + log(e(),100) + [3*(3-3)/3] + (2<3) && 1.23",    &mut ns)?;
    //    |            |      |    |   |          |               |   |
    //    |            |      |    |   |          |               |   boolean logic with short-circuit support
    //    |            |      |    |   |          |               comparisons
    //    |            |      |    |   |          square-brackets act like parenthesis
    //    |            |      |    |   built-in constants: e(), pi()
    //    |            |      |    'log' can take an optional first 'base' argument, defaults to 10
    //    |            |      numeric literal with suffix: p, n, µ, m, K, M, G, T
    //    |            many built-in functions: print, int, ceil, floor, abs, sign, log, round, min, max, sin, asin, ...
    //    standard binary operators

    assert_eq!(val, 1.23);

    Ok(())
}
```
