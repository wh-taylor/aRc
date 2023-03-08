# TODO

- [x] Fix functions
- [ ] Implement function definitions (`f x = ...`)
- [ ] Remove duplicate values in operator results
- [ ] Implement runtime errors
- [ ] Create file runner
- [ ] Make code more organized
- [ ] Embed lua for functions and globals
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
