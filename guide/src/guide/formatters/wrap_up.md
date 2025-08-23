# Wrap Up

Alright! We are done.

Hopefully, by following all these steps, your own custom formatter should build and perform its duty correctly.

In developing the example formatter, I found out that the `crono` specifiers are very specific. Atleast `%.3f`, etc. I tried to use: `%.4f` and it did _not_ like it. Since `3` was too small, I had to go for `6`.
