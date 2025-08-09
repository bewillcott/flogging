<!-- markdownlint-disable-file MD014 -->

# WARNING

This is a **supporting crate** for the `flogging` crate.

It is _not_ meant to be used on its own. In fact, it would not work without the
other crate. Further, it should not be separately added to your project. Add
`flogging` instead, and this will be included as a dependent to that crate.

```text
$ cargo add flogging
```

Alternatively, add the following to your project's `Cargo.toml` file:

```text
[dependencies]
flogging = "0.6.0"
```

Version numbering will be maintained in accordance with the
requirements of the `flogging` crate.
