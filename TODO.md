# TODO

- [x] Fix functions
- [ ] Implement function definitions (`f x = ...`)
- [ ] Remove duplicate values in operator results
- [ ] Implement runtime errors
- [ ] Create file runner
- [ ] Make code more organized
- [ ] Embed lua for functions and globals

## Issues

### Duplicate Values

```
aRc, version 0.0.0
> +/- 1 +/- 1
[-2 : N, 0 : N, 0 : N, 2 : N]

>
```

The expression ±1±1 has four different outcomes:

- `1 + 1`
- `1 - 1`
- `- 1 + 1`
- `- 1 - 1`

These give the results `2`, `0`, `0`, and `-2` respectively. But `0` is repeated twice, so the result can be reduced to a grand total of three solutions: `2`, `0`, and `-2`.

At the moment, the evaluator does not make an effort to remove any duplicates. Pls fix.

### Functions

```
aRc, version 0.0.0
> f = x => x
[function Variable(5, "x") -> Variable(10, "x")]

> x = 1
[1 : N]

> f x

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
zsh: IOT instruction (core dumped)  cargo run
```

An error occurs when the function's parameter and the input constant share the same name.

This is likely due to the function substituting the variable in for the parameter. However, in this case, it simply replaces `x` with `x`, resulting in a loop, which explains the stack overflow.

The same occurs with just using the closure itself:

```
aRc, version 0.0.0
> x = 1
[1 : N]

> (x => x) x

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
zsh: IOT instruction (core dumped)  cargo run
```

Weird. Please fix.