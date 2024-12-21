# Advent of Code [![checks-badge]][checks-link] [![docs-badge]][docs-link]

## Features

* Each solution uses the most efficient algorithms to the best of my knowledge.
* Self contained depending only on the `std` Rust library. No use of `unsafe` features.
* Consistently formatted with `rustfmt` and linted by `clippy`.
* Thoroughly commented with `rustdoc` generated [documentation online][docs-link].
* Test coverage with continuous integration provided by [GitHub Actions][checks-link].

## Quickstart

<details>
<summary>Show details</summary><p/>

**Input**

Place input files in `input/yearYYYY/dayDD.txt` including leading zeroes. For example:
* `input/year2015/day23.txt`
* `input/year2023/day02.txt`

**Run**
* Everything `cargo run`
* Specific year `cargo run year2023`
* Specific day `cargo run year2023::day01`
* Release profile (faster) `cargo run --release`
* Optimized for current CPU architecture (fastest) `RUSTFLAGS="-C target-cpu=native" cargo run --release`

**Test**
* Everything `cargo test`
* Specific year `cargo test year2023`
* Specific day `cargo test year2023::day01`
* Show STDOUT for debugging `cargo test -- --nocapture`

**Benchmark**
* Everything `cargo bench`
* Specific year `cargo bench year2023`
* Specific day `cargo bench year2023::day01`

**Document**
* Build docs including private items `cargo doc --document-private-items`
* Build doc then open HTML landing page `cargo doc --document-private-items --open`

**Miscellaneous**
* Code quality lints `cargo clippy`
* Consistent code formatting `cargo fmt`

</details>

[checks-badge]: https://img.shields.io/github/actions/workflow/status/maneatingape/advent-of-code-rust/checks.yml?label=checks
[checks-link]: https://github.com/maneatingape/advent-of-code-rust/actions/workflows/checks.yml
[docs-badge]: https://img.shields.io/github/actions/workflow/status/maneatingape/advent-of-code-rust/docs.yml?color=blue&label=docs
[docs-link]: https://maneatingape.github.io/advent-of-code-rust/aoc/
[Advent of Code]: https://adventofcode.com
[apple-link]: https://en.wikipedia.org/wiki/Apple_M2
[intel-link]: https://ark.intel.com/content/www/us/en/ark/products/50067/intel-core-i72720qm-processor-6m-cache-up-to-3-30-ghz.html