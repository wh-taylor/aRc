# aRc (Advanced REPL Calculator)

A REPL-based calculator.

## Usage

When the REPL is initiated, you can type in expressions and the calculated answer will be printed.

    aRc, version 0.0.0
    > 1 + 2
    3

    > (4 + 8) / 2
    6

All numbers are stored as rational numbers, so precision is never lost like in some popular programming languages.

    aRc, version 0.0.0
    > 0.1 + 0.2
    0.3

The weird effects of binary in floating-point arithmetic will not occur because instead of these numbers being stored as floats, they're stored as `1 / 10` and `2 / 10`.

But this is not the whole story; numbers also have an imaginary element. So a number like `3` would really be interpreted as `(3 / 1) + (0 / 1)i` in its most expanded form.

An imaginary number can be created by simply multiplying the constant `i`, preprogrammed to `(0 / 1) + (1 / 1)i`, by a number. A complex number can be made by just adding a real number with an imaginary number.

    aRc, version 0.0.0
    > 3 + 2i
    3 + 2i

Since all numbers are stored as complex numbers, complex numbers can perform the same arithmetic as any other number.

    aRc, version 0.0.0
    > 2i + 3i
    5i

    > (5 - (1 / 2)i)(2 + (2 / 3)i)
    31 / 3

An imaginary number can also be made by simply evaluating the square root of -1.

    aRc, version 0.0.0
    > sqrt (-1)
    i

