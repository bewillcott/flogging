<!-- markdownlint-disable-file MD024 MD042 MD033 -->

# Release Log

## Version 0.6.0 [*][0.6.0] - Dev

---
**_Important Notes:_**

- Removed

  ```rust, no_run, noplayground
  impl Handler{
      fn new(){...}
      fn create(name){...}
  }
  ```

- Removed

  ```rust, no_run, noplayground
  Logger::reset_level()
  ```

Both of the above APIs are redundant. So removing them now, before v1.0.0 is released, is best.

- Added a new method to `HandlerTrait` - `set_test_mode()`. This will require updating of any custom handlers.

---

### Great News

Initial release of the online instructional guide: [The FLogging Guide][tfg].

---

Added new methods to `LoggerBuilder`:

- `remove_file()`
  - Use to remove a log file before adding a file handler.
  This is a way of resetting the log file prior to each test run.
- `add_pconsole_handler()` and `add_pconsole_handler_with()`
  - Use to add a production version of the console handler. This handler is different, in that log entries set to `LeveL::INFO`, will have their `msg` printed to `stdout` without any formatting, whilst all other `Level`s will be printed to `stderr` using the set formatter.
- `set_fn_name()`
  - Set the current function/method name. Only required when using the _method_ form of operation instead of the _macro_ form.

Added new method to `Logger` - `pconsole_logger()`.

To facilitate the _pconcole_handlers_, a new enum `ConsoleType` has been added.

Extensive work done on improving both unit and integration tests.

## Version 0.5.0 [*][0.5.0] - 2025-08-03

<details>
<summary>Details (click to see)</summary>

---
**_Important Note:_**

Only applicable for those who have previously created their own custom formatter - `FormatType::Custom(String)` changed to `FormatType::Custom`.

---

Now generally, there were some improvements to the API documentation.

</details>

## Version 0.4.1 [*][0.4.1] - 2025-07-29

<details>
<summary>Details (click to see)</summary>

- Major improvements to the API documentation.
- Increased test coverage to 100%, and included the [Coverage Report].

</details>

## Version 0.4.0 [*][0.4.0] - 2025-07-27

<details>
<summary>Details (click to see)</summary>

This is the initial release.

The reason for not being (0.1.0), is the way I track the internal development
of projects not yet published. However, now that this one is published,
the versioning will progress as expected, in accordance with [Semantic Versioning].

</details>

[0.6.0]: https://github.com/bewillcott/flogging/releases/tag/v0.6.0
[tfg]: https://bewillcott.github.io/flogging/
[0.5.0]: https://github.com/bewillcott/flogging/releases/tag/v0.5.0
[0.4.1]: https://github.com/bewillcott/flogging/releases/tag/v0.4.1
[0.4.0]: https://github.com/bewillcott/flogging/releases/tag/v-0.4.0
[Semantic Versioning]: https://semver.org/
[Coverage Report]: https://bewillcott.github.io/flogging/coverage
