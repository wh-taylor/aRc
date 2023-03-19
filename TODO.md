# TODO

- [x] Fix functions
- [x] Implement runtime errors
- [x] Implement function definitions (`f x = ...`)
- [x] Remove duplicate values in operator results
- [x] Clean up REPL output
- [ ] Implement conditional (`if <cond>`) and matching for functions
- [ ] Implement sets (`{x | x <- R, x >= 0}`, `[0, inf)`, `C`) and set operators (`U`)
- [ ] Add `for` for domain specification
- [ ] Provide more detailed runtime errors
- [ ] Add support for matrices/tuples
- [ ] Add trigonometric functions
- [ ] Optimize operators to not make such big ass numbers during computation
- [ ] Add quaternion type
- [ ] Implement expression simplifier
- [ ] Add float type
- [ ] Add absolute value bar syntax
- [ ] Create file runner
- [ ] Embed lua for functions and globals
- [ ] Add programming functionality (code blocks, loops, etc)
- [ ] Add special commands to REPL (`:_`)
- [ ] Implement function grapher (`:g`)

## Issues

### Functions

#### Function Definition

A user must be able to define a function in a few ways. The first example is the most basic:

```
f = x => x + 2
```

In this case, the value `f` is simply defined as a closure.

The user must also be able to put the input on the left side of the definition as such:

```
f(x) = x + 2
```

By writing the definition above, the value `f` alone must be valid as well.

Multi-variable functions come in two different forms: tuple and currying. Here is a tupled function,

```
f(x, y) = x + y
```

...and here is a curried function:

```
f(x)(y) = x + y
```

Tuple functions can also be curried,

```
f(a, b)(c, d) = a + b + c + d
```

Curried tuple functions can also be tupled.

```
f((a, b), (c, d)) = a + b + c + d
```

This should go on forever.

Paramaters for functions can also have set values in their definitions:

```
f(2) = 3

g(x)(3) = 2 + x

h(3, x) = 2 + x

```

Ideally, functions should be stored as implicit match statements.

```
f(x) = 2x
f(2) = 3
f(3) = 4

// turns into

f = match {
    2: 3
    3: 4
    x: 2x
}
```

```
g(a, b) = a + b
g(x, 1) = x
g(1, y) = 1

// turns into

g = match {
    (a, b): a + b
    (x, 1): x
    (1, y): y
}
```

```
h(a)(b) = a + b
h(x)(1) = x
h(1)(y) = 1

// turns into

h = match {
    a: match {
        b: a + b
    }
    x: match {
        1: x
    }
    1: match {
        y: 1
    }
}
// inputs will search from last-to-first, so h(2)(1) would match into h(x)(1)
// before matching into `a`, but h(2)(2) would try to match into the `x` branch
// but once it realizes that nothing matches there, it tries to match into the
// next branch, which is h(a)(b)
```

The solution is to store all closures as these implicit match statements.

```
(x => 2x) = match { x: 2x }
```

The `match` syntax itself is not required to be implemented in the interface, only internally.

When a function is first defined, define it as the single-branch match statement.

```
f(x) = x + 1
// turns into
f = match { x: x = 1 }
```

When the same function is defined again, take its parameters and turn it into a new branch
of the same function.

```
f(2) = 2
// turns into
f = match {
    x: x = 1
    2: 2
}
```

Matching is done by expressions.

Match branches can be affected by `if` statements:

```
f(x) = 2x
f(x) = 3x if x > 3
// turns into
f = match {
    x: 2x
    x if x > 3: 3x
}
```

#### Function Calling

Whenever a function is defined, it should be stored in a vector accessible by both the
parser and evaluator. Whenever the parser comes across an identifier that matches with
an existing function, the parser should parse it as a function call rather than an
implicit factor.
