<!-- markdownlint-disable-file MD024 MD042 MD033 -->

# Changelog <a href="https://common-changelog.org" title="" target="_blank"><img src="https://common-changelog.org/badge.svg" alt="Common Changelog"></a>

## Version 0.6.0 [*][0.6.0] - Dev

<details>
<summary>Details (click to see)</summary>

### Changed

- **Breaking:** Minor change to the `Iso8601Formatter::fmt_string`. ([`16a7e87`])
- **Breaking:** Minor change to the `SimpleFormatter::fmt_string` and `UnixTimestampFormatter::fmt_string`. ([`5a435f7`])
- Change: `ConsoleHandler` contains `console_type: ConsoleType` instead of ``stderr: bool``.\
  ([`aeceb23`])
- Change: Many updates to the Guide. ([`0d4e738`])

### Added

- **Breaking** Add: New method to `HandlerTrait` - `set_test_mode()`. ([`0fc162b`])
- Add: New `LoggerBuilder` method: `remove_file()`. ([`f9d5b95`])
- Add: New `LoggerBuilder` method: `set_fn_name()`. ([`0a76ce2`])
- Add: `enum ConsoleType`. ([`b0f404b`])
- Add: New `LoggerBuilder` methods: `add_pconsole_handler()` and `add_pconsole_handler_with()`. ([`85bbe5a`])
- Add: New method to `Logger` - `pconsole_logger()`. ([`29bef45`])
- Add: New instructional guide. ([`73d048a`])

### Removed

- **Breaking:** Remove `impl Handler{...}` - `new()` and `create(name)`. ([`3623196`])

### Fixed

- Fix: Many tests due to new and modified API and internal code. ([`0c6aa2f`])

</details>

## Version 0.5.0 [*][0.5.0] - 2025-08-03

<details>
<summary>Details (click to see)</summary>

### Changed

- **Breaking:** Modified `FormatType::Custom(String)` to `FormatType::Custom`. ([`88183d3`])
- Document: Convert inline links to reference links. ([`e5a7bf0`])
- Change: `ConsoleHandler` to contain `stderr: bool` instead of `mod_path: String`. Will now output to `std::io::stderr` if true. ([`41f5e9e`])
- Change: `StringHandler` removed `name: String` and renamed private method `StringHandler::create()` to `StringHandler::new()`. ([`41f5e9e`])

### Added

- Add: Macro and method: `is_logging`. ([`ee882f3`])
- Add: `econsole_logger()` and `add_econsole_handler()`. ([`41f5e9e`])
- Add: `Handler::EConsole`. ([`41f5e9e`])

</details>

## Version 0.4.1 [*][0.4.1] - 2025-07-29

<details>
<summary>Details (click to see)</summary>

### Changed

- Document comments improved or expanded. ([`d35d3e5`])

### Added

- Minimum Rust version now set to: "1.85.1". ([`3c6e99a`])
- Test coverage improved. Now 100%. ([`d35d3e5`])
- [Coverage Report]

### Fixed

- Fix broken documentation links. ([`3e32c90`]) ([`5ca139a`])

</details>

## Version 0.4.0 [*][0.4.0] - 2025-07-27

_Initial release._

[0.6.0]: https://github.com/bewillcott/flogging/releases/tag/v0.6.0
[`0d4e738`]: https://github.com/bewillcott/flogging/commit/0d4e7388040a205bc013385e173cb7efb36e4a4d
[`0c6aa2f`]: https://github.com/bewillcott/flogging/commit/0c6aa2f9521085870144c3e200429133f910469a
[`29bef45`]: https://github.com/bewillcott/flogging/commit/29bef4580276b64cf91bcbb156ce890d27110df6
[`0fc162b`]: https://github.com/bewillcott/flogging/commit/0fc162bf5898c0fe3cc5a8f89c069cfe83baa9c4
[`5a435f7`]: https://github.com/bewillcott/flogging/commit/5a435f75ff9baee54d02c725018561beadc273f6
[`0a76ce2`]: https://github.com/bewillcott/flogging/commit/0a76ce27ce98047045889f24f56684e1d44b9ec7
[`73d048a`]: https://github.com/bewillcott/flogging/commit/73d048a44d1effb496d01213ad549b28bfdf027e
[`85bbe5a`]: https://github.com/bewillcott/flogging/commit/85bbe5a79320f19091abe1f11d06a962f29c2863
[`aeceb23`]: https://github.com/bewillcott/flogging/commit/aeceb233270f7745b2ff7dbcb44d01ce68098f45
[`16a7e87`]: https://github.com/bewillcott/flogging/commit/16a7e87e5a51e8aac4b01a5c6de53ea78ab92439
[`f9d5b95`]: https://github.com/bewillcott/flogging/commit/f9d5b9537960dd23af9f76b77ff5a4b996e777bd
[`b0f404b`]: https://github.com/bewillcott/flogging/commit/b0f404b26858b4b9b1e6839b1769049e6deb2e65
[`3623196`]: https://github.com/bewillcott/flogging/commit/362319610c1f82c8be9ceb7c7d4c3a87637017fa
[0.5.0]: https://github.com/bewillcott/flogging/releases/tag/v0.5.0
[`41f5e9e`]: https://github.com/bewillcott/flogging/commit/41f5e9e047e8cb3fea6cb664f84f9d0f621c89de
[`e5a7bf0`]: https://github.com/bewillcott/flogging/commit/e5a7bf0027c386ad229ea74cfcc3483274e51580
[`88183d3`]: https://github.com/bewillcott/flogging/commit/88183d392edda04b7f7f6bc24165c481991818ef
[`ee882f3`]: https://github.com/bewillcott/flogging/commit/ee882f370a8eb87ef6e152194c869c42a15c19a1
[0.4.1]: https://github.com/bewillcott/flogging/releases/tag/v0.4.1
[`d35d3e5`]: https://github.com/bewillcott/flogging/commit/d35d3e5e8eb0a443b8b71a1f94ba9ea0faca0775
[`5ca139a`]: https://github.com/bewillcott/flogging/commit/5ca139a11ef961f9b48181b76a142eb703e8b34b
[`3e32c90`]: https://github.com/bewillcott/flogging/commit/3e32c9095ecef4994ecb0dd44268d5025010c0cf
[`3c6e99a`]: https://github.com/bewillcott/flogging/commit/3c6e99ae0e38ac5f63540f47176df9fb6667d524
[0.4.0]: https://github.com/bewillcott/flogging/releases/tag/v-0.4.0
[Coverage Report]: https://bewillcott.github.io/flogging/coverage
