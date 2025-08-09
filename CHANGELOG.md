<!-- markdownlint-disable-file MD024 MD042 -->

# Changelog [![Common Changelog](https://common-changelog.org/badge.svg)][cc]

## [0.6.0] - Dev

### Added

- Add: New method for `FloggingBuilder` - `remove_file()`. ([`f9d5b95`])

## [0.5.0] - 2025-08-03

### Changed

- **Breaking:** Modified `FormatType::Custom(String)` to `FormatType::Custom`. ([`88183d3`])
- Document: Convert inline links to reference links. ([`e5a7bf0`])
- Change: `ConsoleHandler` to contain `stderr: bool` instead of `mod_path: String`. Will now output to `std::io::stderr` if true. ([`41f5e9e`])
- Change: `StringHandler` removed `name: String` and renamed private method `StringHandler::create()` to `StringHandler::new()`. ([`41f5e9e`])

### Added

- Add: Macro and method: `is_logging`. ([`ee882f3`])
- Add: `econsole_logger()` and `add_econsole_handler()`. ([`41f5e9e`])
- Add: `Handler::EConsole`. ([`41f5e9e`])

## [0.4.1] - 2025-07-29

### Changed

- Document comments improved or expanded. ([`d35d3e5`])

### Added

- Minimum Rust version now set to: "1.85.1". ([`3c6e99a`])
- Test coverage improved. Now 100%. ([`d35d3e5`])
- [Coverage Report]

### Fixed

- Fix broken documentation links. ([`3e32c90`]) ([`5ca139a`])

## [0.4.0] - 2025-07-27

_Initial release._

[0.6.0]: https://github.com/bewillcott/flogging/releases/tag/v0.6.0
[`f9d5b95`]: https://github.com/bewillcott/flogging/commit/f9d5b9537960dd23af9f76b77ff5a4b996e777bd
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
[cc]: https://common-changelog.org
[Coverage Report]: https://bewillcott.github.io/flogging/
