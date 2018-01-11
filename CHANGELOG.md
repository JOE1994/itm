# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0] - 2018-01-11

### Added

- (library) A `Decoder` abstraction for parsing ITM packets out of `Read`-able sources.
- (library) A `Packet` type that represent ITM packets.
- (cli) The stimulus port to read ITM data from can be changed using the `-s` flag. If the flag is
  omitted the port 0 is used by default.
- (cli) A follow mode (`-F` flag) to keep reading the input file. Use this mode if your OS doesn't
  support named pipes.
- (cli) Support for reading from stdin when no `-f` flag is passed.

### Changed

- [breaking-change][] (cli) the file to read must now be passed as an argument of the `-f` flag

## [v0.1.1] - 2016-10-20

### Changed

- `itmdump` no longer depends on the `mkfifo` command.
- `itmdump`, which normally uses named pipes, now fallbacks to regular files to
  be work on Windows.
- `itmdump` now is restrictive with about the arguments it receives. Before, a
  second argument would simply be ignored, but, now, that has become a hard
  error.
- `itmdump` version output (`-V`) now includes the git commit hash and date.

## v0.1.0 - 2016-10-03

### Added

- `itmdump` tool that parses instrumentation packets from the stimulus port 0
  and dumps the payload to `stdout`.

[Unreleased]: https://github.com/japaric/itm/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/itm/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/japaric/itm/compare/v0.1.0...v0.1.1
