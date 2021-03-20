# `uniqtoo`

A version of `sort | uniq -c` with output that updates in real-time as each line is parsed.


## Usage

Pipe something line-based into `uniqtoo` the same as you would into `sort | uniq -c`.

![An animated example of piping a command into uniqtoo](demo.svg)

You can also supply the input as a file or file descriptor as an argument.

```
$ uniqtoo input.txt
```

## Install

### Rust / Cargo

```
$ cargo install uniqtoo
```

[![Latest version](https://img.shields.io/crates/v/uniqtoo.svg)](https://crates.io/crates/uniqtoo)

The binary will be placed in your `~/.cargo/bin` which should be on your shell's `$PATH`.

### Docker

A container with the binary is available from Docker Hub and GitHub Container Registry.

 * `jakewharton/uniqtoo` [![Docker Image Version](https://img.shields.io/docker/v/jakewharton/uniqtoo)][hub]
 * `ghcr.io/jakewharton/uniqtoo`

[hub]: https://hub.docker.com/r/jakewharton/uniqtoo/

Use `docker run` instead of directly using the binary.

```
input_command | docker run -i -a STDIN -a STDOUT jakewharton/uniqtoo
```

## License

    Copyright 2021 Jake Wharton

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
