# TODO

- [x] Fix functions
- [x] Implement runtime errors
- [x] Implement function definitions (`f x = ...`)
- [ ] Clean up REPL output
- [ ] Implement conditional (`if <cond>`) and matching for functions
- [ ] Implement sets (`{x | x <- R, x >= 0}`, `[0, inf)`, `C`) and set operators (`U`)
- [ ] Add `for` for domain specification
- [ ] Provide more detailed runtime errors
- [ ] Remove duplicate values in operator results
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
