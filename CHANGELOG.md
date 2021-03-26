# Changelog

## [Unreleased]


## [0.3.0] - 2021-03-25

### Added

 - `--limit` argument displays only the given count of lines for emulating the behavior when piping
   into `head`.
 - `--reverse` argument inverts the sort of the output showing the items with the fewest occurrences
   at the top. Combine this with `--limit` to get behavior similar to piping into `tail`.

### Changed

 - Implement `-i` flag for performing case-insensitive grouping (by lowercasing, to match `uniq`).
 - Implement `-f` argument for ignoring the given number of "fields" which are separated by blanks
   (to match `uniq`).
 - Implement `-s` argument for ignoring the given number of characters of each line
   (to match `uniq`).


## [0.2.0] - 2021-03-19

### Added

 - Published a Docker container at `jakewharton/uniqtoo` and `ghcr.io/jakewharton/uniqtoo`.

### Changed

 - `-c` flag is now implied and does not need specified.
 - Removed flags related to functionality of `uniq` that is not specific to `-c`.


## [0.1.0] - 2021-03-19

 - Initial release


[Unreleased]: https://github.com/JakeWharton/uniqtoo/compare/0.3.0...HEAD
[0.3.0]: https://github.com/JakeWharton/uniqtoo/releases/tag/0.3.0
[0.2.0]: https://github.com/JakeWharton/uniqtoo/releases/tag/0.2.0
[0.1.0]: https://github.com/JakeWharton/uniqtoo/releases/tag/0.1.0
